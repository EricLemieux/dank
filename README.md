# Dank

Download memes from reddit in parallel so that you can more efficiently waste time.

## Install

Compiled binaries are available on the [releases page](https://github.com/EricLemieux/dank/releases), or you can build
the binary yourself locally.

```shell
make && sudo make install
```

### MacOS

I have set up a specialized homebrew tap that can be used to install if you don't want to go through the manual
installation process.

```shell
brew tap ericlemieux/tap
brew install ericlemieux/tap/dank
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

Specify the subs that you want to download images from. This should be specified as a comma separated list.

```shell
dank --subs foo,bar,baz
```

## Releasing

* Bump the version in `Cargo.toml`
* Create a tag following the format `vX.Y.Z`, with the release notes as the tags commit message
* A GitHub action will automatically create a new release with the tag
* Update installers, such as the [custom homebrew tap](https://github.com/EricLemieux/homebrew-tap/blob/master/Formula/dank.rb)
