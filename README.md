# coinfo

A CLI tool that provides useful information about cryptocurrencies.

## Installation

```
$ cargo install coinfo
```

## Usage

### List tickers

```
$ coinfo ticker BTC,ETH

+---------+----------+--------------+--------------+-------------------+
| Symbol  | Exchange | Price (USDT) | PriceChange% | 24H Volume (USDT) |
+======================================================================+
| BTCUSDT | Binance  | 20623.11     | -4.222       | 5157235192.589115 |
|---------+----------+--------------+--------------+-------------------|
| ETHUSDT | Binance  | 1572.3       | -7.519       | 1560911036.891572 |
+---------+----------+--------------+--------------+-------------------+
```

### List community info

```
$ coinfo community btc

+----------+------------------------+---------------------------------+------------------------------------+------------------------------------------------+
| Currency | Homepage               | Explorer                        | Opensource                         | Description                                    |
+===========================================================================================================================================================+
| btc      | http://www.bitcoin.org | https://blockchair.com/bitcoin/ | https://github.com/bitcoin/bitcoin | Bitcoin is the first successful internet money |
|          |                        | https://btc.com/                | https://github.com/bitcoin/bips    | based on peer-to-peer technology; whereby no   |
|          |                        | https://btc.tokenview.com/      |                                    | central bank or authority is involved in the   |
|          |                        |                                 |                                    | transaction and production of the Bitcoin      |
|          |                        |                                 |                                    | currency. It was created by an anonymous       |
|          |                        |                                 |                                    | individual/group under the name, Satoshi       |
|          |                        |                                 |                                    | Nakamoto. The source code is available         |
|          |                        |                                 |                                    | publicly as an open source...                  |
|          |                        |                                 |                                    |                                                |
|          |                        |                                 |                                    | Read more on                                   |
|          |                        |                                 |                                    | https://www.coingecko.com/en/coins/bitcoin     |
+----------+------------------------+---------------------------------+------------------------------------+------------------------------------------------+
```

### List airdrops

```
$ coinfo airdrop --status ongoing

+---------------------------------------+----------+--------------+-------------------+----------------------+---------------------+---------------------+--------------------------------------+
| Ongoing Project                       | Currency | Participated | Number of winners | Total airdrop amount | Start date (UTC)    | End date (UTC)      | Link                                 |
+===============================================================================================================================================================================================+
| SENSO Token $75 000 Airdrop Campaign  | SENSO    | 80940        | 2000              | 384498               | 2022-08-10 16:00:00 | 2022-08-30 16:00:00 | https://coinmarketcap.com/currencies |
|                                       |          |              |                   |                      |                     |                     | /senso/airdrop/                      |
+---------------------------------------+----------+--------------+-------------------+----------------------+---------------------+---------------------+--------------------------------------+

```

### Help

```
$ coinfo --help

USAGE:
    coinfo <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    community
    help         Print this message or the help of the given subcommand(s)
    ticker

$ coinfo ticker --help

USAGE:
    coinfo ticker [OPTIONS] <CURRENCIES>

ARGS:
    <CURRENCIES>

OPTIONS:
    -e, --exchange <EXCHANGE>
    -h, --help                   Print help information

$ coinfo airdrop --help

USAGE:
    coinfo airdrop [OPTIONS]

OPTIONS:
    -h, --help               Print help information
    -s, --status <STATUS>    [possible values: ongoing, upcoming, ended]
```

## TODO

- [x] Airdrops Information
