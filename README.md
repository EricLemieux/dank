# Dank

Download memes from reddit in parallel so that you can more efficiently waste time.

## Install

```shell
make && sudo make install
```

## Usage

Download memes and then the user can view them however they want.

```shell
dank
```

Download to a specific directory, useful in the case of ~~data hoarding~~ archiving.

```shell
dank /path/to/my/archives/$(date "+%Y-%m-%d")
```

Download memes and start a simple web server, so you can access from your web browser.

```shell
dank --serve
```