"""
Transforms and Loads data into the local SQLite3 database
"""

import sqlite3
import csv


# load the csv file and insert into a new sqlite3 database
def load(dataset="../data/US_births.csv"):
    """Transforms and Loads data into the local SQLite3 database"""
    payload = csv.reader(open(dataset, newline=""), delimiter=",")
    # skips the header of csv
    next(payload)
    conn = sqlite3.connect("../data/US_births_DB.db")
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS US_births_DB")
    c.execute(
        """
        CREATE TABLE US_births_DB (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            year INTEGER,
            month INTEGER,
            date_of_month INTEGER,
            day_of_week INTEGER,
            births INTEGER
        )
    """
    )
    # insert
    c.executemany(
        """
        INSERT INTO US_births_DB (
            year,
            month,
            date_of_month,
            day_of_week,
            births
            ) 
            VALUES (?, ?, ?, ?, ?)""",
        payload,
    )
    conn.commit()
    conn.close()
    return "../data/US_births_DB.db"
