grep -> the filter
     -> searches for patterns(regex)
awk -> the surgeon
    -> processess columns and fields. if a log file is a table. 'awk' picks the columns
sort -> the organizer
     -> puts lines in alphabetical or numerical order(essential for 'uniq' tool)
uniq -> the de-duplicator
     -> removes or counts duplicate lines(only works on sorted data)
wc -> the counter
   -> count lines(-1), words, or bytes.


Building a pipeline:
    scenario : you have a raw log file of IP addresses hitting your server. you need to find the TOP 5 IP ADDRESSES that have the most failed login attempts.

    step-by-step construction:
        1. start with the data
            cat access.log
        2. filter for failures
            grep "FAILED LOGIN"
        3. extract the IP address(assuming the IP is in the 3rd column)
            awk '{print $3}'
        4. sort them(required so identical IPs are adjacent)
            sort
        5. count occurrences
            uniq -c  :: this outputs [count] [IP_address]
        6. sort by the count
            sort -nr  :: 'n' for numerics, '-r' for reverse/highest first
        7. take the top 5
            head -n 5

        pipeline::
            cat access.log | grep "FAILED LOGIN" | awk '{print $3}' | sort | uniq -c | sort -nr | head -n 5


Advanced Dissection: 'awk' and 'sed'
    'awk' -> column manipulation
          -> views a line as a set of fields separated by spaces(default)

          awk '{print $1, $NF}'  :: prints the FIRST field and the LAST field of every line.
          awk -F: '{print $1}' /etc/password   :: the '-F' tells awk the "separator"is a colon instead of a space.

    'wc -1' ->the sanity check
            -> when building a pipeline, always end with 'wc -1' to see how many results you have before you print them to the screen.
            -> eg; 'ls /var/log | wc -1' ::shows how many log files exists.


