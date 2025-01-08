import argparse
import sqlite3

parser = argparse.ArgumentParser(
    description="Create a SQLite database with a users table."
)
parser.add_argument(
    "db",
    metavar="DATABASE",
    type=str,
    help="The name of the SQLite database file to create.",
)
args = parser.parse_args()

DATABASE_FILE = args.db
# Connect to the SQLite database (creates the file if it doesn't exist)
conn = sqlite3.connect(DATABASE_FILE)

# Create a cursor object to execute SQL commands
cursor = conn.cursor()

# Define the SQL command to create the users table
create_table_query = """
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,               -- Primary key as a string
    name TEXT NOT NULL,                -- Name as a string
    hashed_password TEXT NOT NULL,     -- Hashed password as a string
    role TEXT NOT NULL CHECK (role IN ('user', 'admin')), -- Role with a constraint
    created DATETIME DEFAULT CURRENT_TIMESTAMP -- Creation timestamp
);
"""

# Execute the create table command
cursor.execute(create_table_query)

# Commit the changes and close the connection
conn.commit()
conn.close()

print(f"Database '{DATABASE_FILE}' is created.")
