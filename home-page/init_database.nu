#!/usr/bin/env nu

def main [db_name: string] {
    stor create --table-name users --columns {name: str, hashed_password: str, role: str, created: datetime}
    stor export -f $db_name
}