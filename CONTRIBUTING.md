# Contributing

You want to help contribute? Awesome! Thanks for taking the time to look at the
guidelines for this repo. Here's what you need to know!

## License

**yep** is proudly licenced under the MIT license, and so are all
contributions. Please see the [`LICENSE`] file in this directory for more details.

[`LICENSE`]: https://github.com/jankatins/yep/blob/main/LICENSE

## Pull Requests

To make changes to **yep**, please send in pull requests on GitHub to
the `main` branch. I'll review them and either merge or request changes. GitHub Actions
tests everything as well, so you may get feedback from it too.

If you make additions or other changes to a pull request, feel free to either amend
previous commits or only add new ones, however you prefer. I may ask you to squash
your commits before merging, depending.

## Issue Tracker

You can find the issue tracker [on
GitHub](https://github.com/jankatins/yep/issues). If you've found a
problem with **yep**, please open an issue there.


## Examples
Do you want to help show off some ways for how the library works? Feel free to
work on an example and open up a PR!

[install Rust]: http://rust-lang.org/install.html

To run the tests:

```bash
$ cargo test
```

## Types of Contributions

### Report Bugs

Report bugs at https://github.com/jankatins/yep/issues.

If you are reporting a bug, please include:

* Your operating system name and version.
* Any details about your local setup that might be helpful in troubleshooting.
* Detailed steps to reproduce the bug.

### Fix Bug

Look through the GitHub issues for bugs. Anything tagged with "bug"
is open to whoever wants to implement it.

### Implement Features

Look through the GitHub issues for features. Anything tagged with "feature"
is open to whoever wants to implement it.

### Write Documentation

yep could always use more documentation, whether as part of the
official yep docs, in docstrings, or even on the web in blog posts,
articles, and such.

### Submit Feedback

The best way to send feedback is to file an issue at https://github.com/jankatins/yep/issues.

If you are proposing a feature:

* Explain in detail how it would work.
* Keep the scope as narrow as possible, to make it easier to implement.
* Remember that this is a volunteer-driven project, and that contributions
  are welcome :)

## Get Started!

Ready to contribute? Here's how to set up `yep` for local development.

1. Fork the `yep` repo on GitHub.
2. Clone your fork locally:
    ```shell
    $ git clone git@github.com:your_name_here/yep.git
    ```
3. Create a branch for local development:
    ```shell
    $ git checkout -b name-of-your-bugfix-or-feature
    ```
   Now you can make your changes locally.

4. When you're done making changes, check that your changes pass the tests:
    ```shell
    $ cargo test
    ```
5. Commit your changes and push your branch to GitHub:
    ```shell
    $ git add .
    $ git commit -m "Your detailed description of your changes."
    $ git push origin name-of-your-bugfix-or-feature
    ```
6. Submit a pull request through the GitHub website.

## Pull Request Guidelines

Before you submit a pull request, check that it meets these guidelines:

1. The pull request should include tests.
2. If the pull request adds functionality, the docs should be updated. Put
   your new functionality into a function with a docstring, and add the
   feature to the list in README.md.
