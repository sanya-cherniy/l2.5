#!/ bin / bash

SECONDS=0


SUCCESS=0
FAIL=0
part_2=$(cd part_2 && ls)

if [ "$LANG" == "ru_RU.UTF-8" ]
then
    CHECK="Файлы test_real.txt и test_to_check.txt идентичны"
else
    CHECK="Files test_real.txt and test_to_check.txt are identical"
fi

#### PART 2
# 1 flag, 1 pattern, 1 file

for file in $part_2
do
    for pattern in $(cat patterns.txt)
    do
        for flag in -A2 -B2 -C2 -v -c -i -n -F
        do

            grep  $pattern $flag part_2/"$file" > test_real.txt
            ./target/debug/my_grep $pattern $flag part_2/"$file" > test_to_check.txt
            RES=$(diff -s test_real.txt test_to_check.txt)
            if [ $? -eq 0 ]
            then
            (( SUCCESS++ ))
            echo " $pattern $flag $file ACCEPTED"
            else
            (( FAIL++ ))
            echo " $pattern $flag $file FAILED"
            fi
            rm -rf test_real.txt test_to_check.txt
        done
    done
done

flags=(-A2 -B2 -C2 -v -c -i -n -F)
start=1
for file in $part_2
do
    for file_2 in $part_2
    do
            if [ $file != $file_2 ]
            then
        for pattern in $(cat patterns.txt)
        do
                for (( i=0; i <= 7; i++))
                do
                    for (( j=start; j <= 7; j++ ))
                    do
                    if [ ${flags[i]} != ${flags[j]} ]
                    then
                    grep  $pattern ${flags[i]} ${flags[j]} part_2/"$file_2" part_2/"$file" > test_real.txt
                    ./target/debug/my_grep  $pattern ${flags[i]} ${flags[j]} part_2/"$file_2" part_2/"$file" > test_to_check.txt
                    RES=$(diff -s test_real.txt test_to_check.txt)
                    if [ $? -eq 0 ]
                    then
                    (( SUCCESS++ ))
                     echo " $pattern ${flags[i]} ${flags[j]} $file $file_2 ACCEPTED"
                    else
                    (( FAIL++ ))
                    echo " $pattern ${flags[i]} ${flags[j]} $file $file_2 FAILED"
                    fi
                    rm -rf test_real.txt test_to_check.txt
                    fi
                    done
                done
                ((start++))

        done
        start=2
        fi
    done
done






duration=$SECONDS

echo "$(($duration / 60)) minutes and $(($duration % 60)) seconds elapsed."

echo "$SUCCESS <- ACCEPTED"
echo "$FAIL <- FAILED"
echo "`expr $SUCCESS + $FAIL` <- ALL"
