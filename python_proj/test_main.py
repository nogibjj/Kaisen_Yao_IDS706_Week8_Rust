"""
Test goes here

"""

from mylib.query import query


def test_query():
    result = query()
    assert result == "Success"


if __name__ == "__main__":
    test_query()
