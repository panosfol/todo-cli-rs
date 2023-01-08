# todo-cli
<p>A cli for managing todos

You can:
- Create a new entry
- Edit your entry(title, description and/or category of entry)
- Delete your entry
- Change the status of the entry
- Fetch and read all the entries

## Prerequisites
1. [Rust](https://www.rust-lang.org/tools/install)
2. [Docker](https://docs.docker.com/engine/install/)
3. [Diesel](https://diesel.rs/guides/getting-started)

## Setup
### Run a MySqlDatabase from Docker
### Run database setup & migrations

```sh
diesel setup
diesel migrations generate
```

### Use connect --url <string> command with the url as argument. Read .env.example for the correct format

## Example commands
#### Creating new entry with and without the category given

```sh
new
new -c Fun
```

#### Reading all the entries filtered throught status and/or category if provided

```sh
ls (or list)
ls -c Fun
ls -s Done
ls -c Personal -s Active
```

#### Edit the title, the description and/or the category of a specific entry

```sh
edit
```

#### Delete an entry

```sh
del (or delete)
```

#### Change the status of an entry, chose from a drop down menu

```sh
status 
```

#### Connect with the database. This is to be used once after installing the todo-cli

```sh
connect --url <string> (string is the url to connect to the database, see setup for instructions)
```




