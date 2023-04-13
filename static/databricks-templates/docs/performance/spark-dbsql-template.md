# Spark DBSQL Query Performance Issue Template

The objective of this template is to help you create a good issue for the Spark DBSQL Query Performance Issue Template. 
Please follow the instructions below and delete this text before submitting your issue.

## Things to include but Databricks Solutions Architects cannot access

1. Databricks Solutions Architects cannot access the following urls if you chose to send (please do so, but we cannot access)
    * Notebook url
    * Job run url

## Spark Job (Batch ETL Job with workflows)

1. Databricks Runtime Version (Screenshot)
2. Cluster Size and VM Type (Screenshot)
3. Is photon enabled (Screenshot)
4. Spark Settings, please scrub out any keys (Screenshot)
5. Init Scripts, (Screenshot)
6. Are you running any .collect(), .toPandas() or running a lot of counts in the table?
7. Ganglia Metrics (Screenshot)
8. Snippet of the code that is causing the issue (if possible)
9. Snippet of the logs (if possible)
10. Explain plan by running EXPLAIN EXTENDED on the query (if possible)


### Super Important: Spark UI Screenshots

You must do this on a separate cluster or a job run otherwise there will be potentially other jobs running and skewing the results

1. Spark UI Job Page, sort by duration and take a screenshot of the offenders
2. Spark UI Stage Page, and show us if there is any spil to disk at the top
3. Spark UI SQL graph and click expand all details and click the button that says download screen as PNG


### Super Important: Storage information

1. How many files are in the table (if its delta table you can run `DESCRIBE DETAIL <table_name>`)
2. What is the query history and operational metrics on the table (if its delta table you can run `DESCRIBE HISTORY <table_name>`)
3. If it is parquet just try to get the number of files and the size of atleast one file.
4. What is the # of columns in the table

## Example Email:


