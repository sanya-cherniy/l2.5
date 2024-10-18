use clap::{Arg, ArgMatches, Command};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    // Инициализируем аргументы
    let matches = args_init();
    // Получаем имена файлов
    let files: Vec<String> = matches
        .get_many::<String>("files")
        .unwrap()
        .cloned() // Создаем клоны значений, так как get_many возвращает ссылки
        .collect();
    // Проверяем каждый файл на существование и выводим результат
    for file in files {
        match File::open(&file) {
            Ok(value) => file_checker(value, &matches),
            Err(_) => println!("{}: No such file or directory", file),
        };
    }
}

// Инициализация аргументов командной строки
fn args_init() -> ArgMatches {
    let matches = Command::new("My sort")
        .version("1.0")
        .about("grep analog")
        .arg(
            Arg::new("after")
                .short('A')
                .long("after-context")
                .help(
                    "Print NUM lines of trailing context after matching lines.  Places
              a  line  containing  a  group  separator  (--) between contiguous
              groups of matches.",
                )
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("before")
                .short('B')
                .long("before-context")
                .help(
                    "Print NUM lines of leading context before matching lines.   Places
              a line containing a group separator (--) between contiguous groups
              of  matches.",
                )
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("context")
                .short('C')
                .long("context")
                .help(
                    "Print NUM lines of output context.  Places  a  line  containing  a
              group  separator  (--) between contiguous groups of matches.",
                )
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help(
                    "Suppress  normal  output;  instead print a count of matching lines
              for each input file.  With  the  -v,  --invert-match  option  (see
              above), count non-matching lines.",
                )
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("ignore-case")
                .short('i')
                .long("ignore-case")
                .help(
                    "Ignore  case  distinctions  in  patterns  and  input data, so that
              characters that differ only in case match each other.",
                )
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("invert")
                .short('v')
                .long("invert-match")
                .help("Invert the sense of matching, to select non-matching lines.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fixed")
                .short('F')
                .long("fixed-strings")
                .help("Interpret PATTERNS as fixed strings, not regular expressions.")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("line-num")
                .short('n')
                .long("line-number")
                .help(
                    "Prefix each line of output with the 1-based line number within its
              input file.",
                )
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("patterns")
                .help("searches for PATTERNS")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(true),
        )
        .arg(
            Arg::new("files")
                .help("in each FILEin each FILE")
                .value_name("FILE")
                .required(true)
                .action(clap::ArgAction::Append),
        )
        .get_matches();
    return matches;
}

// Функция которая просматривает файл и выводит результат
fn file_checker(input: File, matches: &ArgMatches) {
    let reader = BufReader::new(input); // Считываем файл в память
    let str_vec: Vec<String> = reader.lines().map(|x| x.unwrap()).collect(); //Преобразуем в массив строк
    let mut count: u32 = 0;
    let mut expression = matches.get_one::<String>("patterns").unwrap().clone(); // Получаем выражение
    let mut after: usize = 0;
    let mut before: usize = 0;
    if matches.contains_id("after") {
        after = *matches.get_one::<usize>("after").unwrap(); // Кол-во строк для вывода после найденной строки (флаг -A)
    }
    if matches.contains_id("before") {
        before = *matches.get_one::<usize>("before").unwrap(); // Кол-во строк для вывода перед найденной строкой (флаг -B)
    }
    if matches.contains_id("context") {
        after = *matches.get_one::<usize>("context").unwrap(); // Кол-во строк для вывода после и перед найденной строкой (флаг -B)
        before = after;
    }
    if matches.get_flag("ignore-case") {
        // Если включен флаг -i преобразуем выражение для игнорирования регистра
        expression = format!("(?i){}", expression);
    }
    let re = Regex::new(&expression).unwrap(); // Компилируем выражение

    for i in 0..str_vec.len() {
        // Проходим по каждой строке файла
        let is_match;

        // Если включен флаг -v то выводить необходимо строки не соответствующие регулярному выражению (кроме тех строк которые выводятся посредством включения флагов -A, -B, -C)
        if matches.get_flag("invert") {
            is_match = !re.is_match(str_vec[i].as_str());
        } else {
            is_match = re.is_match(str_vec[i].as_str()); // Функция возвращает true если переданная строка соответствует регулярному выражению, результат компиляции которого хранится в "re"
        }
        if is_match {
            // Если в строке найдена подстрока соответствующая регулярному выражению
            if matches.get_flag("count") {
                // Если включен флаг -c - только увеличиваем счетчик
                count += 1;
            } else {
                if matches.contains_id("before") || matches.contains_id("context") {
                    // Если включен флаг -C или -B, необходимо напечатать n строк до найденной
                    let mut before_strings = vec![];

                    for j in (usize_max_sub(i, before)..=usize_max_sub(i, 1)).rev() {
                        // В цикле проходим строки от i-n включительно до i не включительно в обратном порядке

                        // Если находим строку которая тоже соответствует регулярному выражению - выходим из цикла (для флага -v - наоборот) необходимо напечатать n строк перед найденной, не соответствующих выражению либо, если меньше чем через n строк найдена строка которая соответствует выражению (для флага -v наоборот) - вывести все строки вплоть до найденной
                        if matches.get_flag("invert") {
                            if !re.is_match(str_vec[j].as_str()) {
                                break;
                            }
                        } else {
                            if re.is_match(str_vec[j].as_str()) {
                                break;
                            }
                        }

                        // Добавляем найденную строку в вектор
                        // Если включен флаг -n - добавляем перед выводимой строкой ее номер
                        if matches.get_flag("line-num") {
                            before_strings.push(format!("{}-{}", j + 1, str_vec[j].clone()));
                        } else {
                            before_strings.push(str_vec[j].clone());
                        }
                    }
                    // Выводим найденные строки в обратном порядке
                    for before_string in before_strings.iter().rev() {
                        println!("{}", before_string);
                    }
                }

                // Выводим текущую строку
                if matches.get_flag("line-num") {
                    print!("{}:", i + 1);
                }
                println!("{}", str_vec[i]);

                // Если включен флаг -C или -A, необходимо напечатать n строк после найденной, не соответствующих выражению либо, если меньше чем через n строк найдена строка которая соответствует выражению (для флага -v наоборот) - вывести все строки вплоть до найденной
                if matches.contains_id("after") || matches.contains_id("context") {
                    for j in 1..=after {
                        if str_vec.len() <= i + j {
                            // Если выходим за границы массива - заканчиваем цикл
                            break;
                        }
                        // Проверяем, соответствует ли строка выражению
                        if matches.get_flag("invert") {
                            if !re.is_match(str_vec[i + j].as_str()) {
                                break;
                            }
                        } else {
                            if re.is_match(str_vec[i + j].as_str()) {
                                break;
                            }
                        }

                        if matches.get_flag("line-num") {
                            print!("{}-", i + j + 1);
                        }
                        println!("{}", str_vec[i + j]);
                    }
                }
            }
        }
    }

    if matches.get_flag("count") {
        println!("{}", count);
    }
}
// Вспомогательная фунция для нахождения наибольшей разницы usize без переполнения
fn usize_max_sub(x: usize, y: usize) -> usize {
    let mut sub = y;
    while sub > 0 {
        match x.checked_sub(sub) {
            Some(value) => return value,
            None => sub -= 1,
        }
    }
    return sub;
}
