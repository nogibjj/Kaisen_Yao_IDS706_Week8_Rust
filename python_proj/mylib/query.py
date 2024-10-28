"""Query the database"""

import sqlite3


def query():
    """Query the database and perform insert, update, and delete operations"""
    conn = sqlite3.connect("../data/US_births_DB.db")
    cursor = conn.cursor()

    # Perform SELECT
    cursor.execute("SELECT * FROM US_births_DB")

    # Perform INSERT
    cursor.execute(
        """
        INSERT INTO US_births_DB (year, month, date_of_month, day_of_week, births) 
        VALUES (2008, 8, 8, 1, 9999)"""
    )

    # Perform UPDATE
    cursor.execute(
        """
        UPDATE US_births_DB 
        SET day_of_week = 1, births = 6666
        WHERE id = 1"""
    )

    # Perform DELETE
    cursor.execute(
        """
        DELETE FROM US_births_DB 
        WHERE id = 2"""
    )

    conn.commit()  # Ensure all changes are saved
    conn.close()
    return "Success"  # Return success for testing
