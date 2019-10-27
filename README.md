# regular-expression-tests

This created a dataset of a bunch of regular expressions run against strings of numbers.
You can see the results of the tests in final_data.txt.
Don't actually use this data, there are a few issues, such as the shell expanding
certain strings like ?1 and ?0, which caused some errors in measurement.

To get the dataset that I looked at, do 

```
$ git clone https://github.com/AlexDikelsky/regular-expression-tests
$ cd regular-expression-tests/clearer_algorithm
$ ./runall.sh
```

Now you should have a file called `data.txt` that contains the data I used.
