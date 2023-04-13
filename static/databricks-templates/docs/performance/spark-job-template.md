# Spark Job Performance Issue Template

The objective of this template is to help you create a good issue for the Spark Job Performance Issue Template. 
Please follow the instructions below and delete this text before submitting your issue.

## Things to include but we cannot access

1. Databricks Solutions Architects cannot access the following urls if you chose to send (please do so, but we cannot access)
    * Notebook url
    * Job run url

## Spark Job (Batch ETL Job with workflows)

??? "1. Databricks Runtime Version"

    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla et euismod
    nulla. Curabitur feugiat, tortor non consequat finibus, justo purus auctor
    massa, nec semper lorem quam in massa.

??? "2. Cluster Size and VM Type"

    ![Image title](https://dummyimage.com/600x400/)


??? "3. Is photon enabled"
   
    ![Image title](https://dummyimage.com/600x400/)

??? "4. Spark Settings"

    ![type:video](https://www.youtube.com/embed/LXb3EKWsInQ)

??? "5. Init Scripts"

    ![type:video](https://www.youtube.com/embed/LXb3EKWsInQ)

??? "6. Are you running any .collect(), .toPandas() or running a lot of counts in the table?"

    ![type:video](https://www.youtube.com/embed/LXb3EKWsInQ)

??? "7. Ganglia Metrics"

    ...

??? "8. Snippet of the code that is causing the issue (if possible)"

    ...

??? "9. Export of the logs (if possible)"

    ...

??? "10. Explain plan by running EXPLAIN EXTENDED on the query (if possible)"

    ...


### Super Important: Spark UI Screenshots

You must do this on a separate cluster or a job run otherwise there will be potentially other jobs running and skewing the results

1. Spark UI Job Page, sort by duration and take a screenshot of the offenders
2. Spark UI Stage Page, and show us if there is any spil to disk at the top
3. Spark UI SQL graph and click expand all details and click the button that says download screen as PNG

??? "1. Spark UI Job Page & please sort it by duration and take a screenshot of the offenders"

    ![Image title](https://dummyimage.com/600x400/)

??? "2. Spark UI Stage Page, and show us if there is any spill to disk at the top and the amount of shuffle"

    ![Image title](https://dummyimage.com/600x400/)

### Super Important: Storage information

1. How many files are in the table (if its delta table you can run `DESCRIBE DETAIL <table_name>`)
2. What is the query history and operational metrics on the table (if its delta table you can run `DESCRIBE HISTORY <table_name>`)
3. If it is parquet just try to get the number of files and the size of atleast one file.
4. What is the # of columns in the table

## Example Email:


