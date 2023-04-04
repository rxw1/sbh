Session Buddy Helper
====================

`Command line tool to do various things with your Session Buddy database`

What
----

[Session Buddy](https://sessionbuddy.com/) is a Google Chrome [extension](https://chrome.google.com/webstore/detail/session-buddy/edacconmaakjimmfgnblocblbcdcpbko) to store and restore browser tabs. All its data is stored in a SQLite3 database. There is a Google Group where some discussion about Session Buddy takes place and, it seems, people sometimes lose their data or are having problems exporting or importing their stuff. This utility aims to do various operations on that database and help out those in need.

Features
--------

* **Backup:** Create a JSON file similar to what the extension would do. The produced output is not exactly the same, but should be viable to be imported into Session Buddy again.

* **Import:** Import a backup file created by either the extension or this tool into a database.

* **Search:** Search your disk for Session Buddy databases. Sometimes it's a little bit cumbersome to figure out the path to the extension's database, so this should make things easier.

* **New:** Create a new and empty Session Buddy database. It will have the same schema as when created by the extension.

* **Debug:** Try to figure out if something is wrong with a database or a backup file.

* **Stats:** Print various stats about a database. Useful to figure out what happened after executing some other task on the database.

* **Dump:** Print all links to stdout.

Current State & Motivation
--------------------------

So far it seems to work.

Over the years I have aggregated multiple databases with in summary around 5,000 stored sessions consisting of around 15,000 windows and more than 130.0000 tabs. The extension would choke on exporting or just crash the extension or make the browser itself freeze. By doing all the stuff on the SQLite database itself, all that works.

But still, it may be not advisable to carry all your stuff around in a Chrome extension all the time. By scheduling regular backups and pruning operations on the database Session Buddy probably won't lose it and, you have convenient (and performant) access to all your data.

If you have any ideas or wishes regarding this tool or need help rescuing your thousands of tabs, feel free to reach out in the [discussions](https://github.com/rxw1/sbh/discussions).

Installation
------------

```
cargo install sbh
```

```
git clone https://www.github.com/rxw1/sbh
cd sbh
cargo install --release .
```

```sh
curl -LSfs https://japaric.github.io/trust/install.sh |\
 sh -s -- --git rxw/sbh
```

Or download precompiled packages from [here](http://www.github.com/rxw1/sbh/releases).

Usage Examples
--------------

### Search for databases

Search for databases and print out the paths. By default, we're trying to figure out the proper path automatically depending on the operating system. For more information about these paths have a look at [dirs-rs](https://github.com/dirs-dev/dirs-rs).

```
sbh search
```

Or you can pass any path to search there:

```
sbh search ~/.config
```

### Backup a database to JSON

```sh
sbh backup -o whatever.json
```

If you do not specify an output file, the produced JSON will be printed to the standard output.

### Search and backup each found database to a timestamped file

```sh
for db in "$(sbh search)"; do
  o=sb-$(sbh id $db)-$(date +%Y%m%d%H%M%S).json
  sbh backup -o $o $db
  sbh validate backup $o
done
```

### Dump sort and count all URLs stored in a database

```sh
sbh search | while read -r l; do
  sbh dump "$l" | grep -Ev 'chrome(|-.*)://' | sort | uniq -c | sort -g
done
```

TODO (Maybe)
-----------

* CSV export
* Prune the database using certain criteria
* Have config file to configure stuff
* Handle `CurrentSession` and `PreviousSession` sessions

Is it any good?
---------------

Yes, of course. It is written in Rust, and it's blazingly fast.

Findings
--------

* For whatever reason, the JSON backups produced by the extension have `<feff>` as the first byte, i.e. a `BOM character` aka `Zero Width No-Break Space`, which breaks it for most parsers. I don't think having this in the beginning of a JSON file is a good thing. After removal, everything is fine. Here's some discussion about that on Stack Overflow: [https://stackoverflow.com/q/4990095/220472](https://stackoverflow.com/q/4990095/220472).

Disclaimer
----------

Neither me or this tool is in any way affiliated with the developers of Session Buddy.