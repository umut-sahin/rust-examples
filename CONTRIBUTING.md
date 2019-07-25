<div align="center">
  <h1>Contribution Guidelines</h1>
  First and foremost, thank you! Your time is valuable, and your contributions mean a lot.
</div>

## Important notice

By contributing to this project:

- You agree that you have authored 100% of the content.
- You agree that you have the necessary rights to the content.
- You agree that you have received the necessary permissions from your employer to make the contributions (If applicable).
- You agree that the content you contribute may be provided under the Projects license.
- You agree that you read the [code of conduct] and to act accordingly.

## How can you contribute?

There are several ways to contribute to this repository:

- [Submitting feature requests](#submitting-feature-requests)
- [Reporting issues](#reporting-issues)
- [Fixing issues](#development-workflow)
- [Improving existing examples](#development-workflow)
- [Adding new examples](#development-workflow)

## Submitting feature requests

You can submit feature requests to improve existing examples or to add new examples.

Before submitting feature requests, make sure you are using the latest version of the repository as the feature you wish to have could be already added.
If you are using the latest version, please use the [GitHub] search to check if a similar feature request is already present.
If you've found a feature request that is:

- **Open:** Drop a comment to describe the additional things you wish.
- **Missing:** [Open a new feature request].

You can use the [FEATURE_REQUEST] template to submit feature requests (You may remove the name and about part. Those are for [GitHub]).

## Reporting issues

Before creating an issue, make sure that you are using the latest version of the repository as the issue you wish to report could be already resolved.
If you are using the latest version, please use the [GitHub] search to check if the issue is already known.
If you've found an issue that is:

- **Closed:** Check if the issue provides a solution for your issue.
  If it's already fixed, it could be that there has been a regression in the code.
  In this case, it's best to open a new issue.
  For all other cases, it might make more sense to add a comment to the closed issue explaining that you're still affected by this.
- **Open:** Try to provide more details about the issue.
  If you can reproduce the issue in a different way than the one used by the original author, please add this.
  The more ways there is to reproduce the bug, the more are the chances to get it fixed quickly.
- **Missing:** [Open a new bug report].
  While doing so, to save everyone time and make it much more likely for your issue to be understood, worked on and resolved quickly, it would help if you're mindful of [How to Report Bugs Effectively] when pressing the "Submit new issue" button.

You can use the [BUG_REPORT] template to report bugs (You may remove the name and about part. Those are for [GitHub]).

## Development workflow

For all the other ways of contributing, please check if there is an open issue for the thing you want to work on.
If the issue does not exist, first [open a new issue] and wait for the feedback.

Once the issue is discussed or if the issue is already discussed, you need to fork the repository and install dependencies to start contributing.
You can do this with the following commands.

```
$ git clone https://github.com/umut-sahin/rust-examples.git
$ cd rust-examples
$ git checkout development
```

Now you are ready to make changes.

After you are happy with your changes, you need to commit your changes to git.
Please write descriptive commit messages.
You can do this with the following command.

```
$ git commit -m "INSERT_DESCRIPTIVE_COMMENT"
```

Commit as much as you like.
After you are confident with your work, push your changes to your fork with the following command.

```
$ git push origin development
```

Then, open a **_descriptive_** pull request to the original repository's **_development_** branch using the [PULL_REQUEST_TEMPLATE] template (Pull requests to master will be closed).

After that, the changes you made will be reviewed and given feedback.
Once everybody agrees that the code is ready to be merged to master, a maintainer will make necessary changes and merge the development branch with master.

## Notes

### General

- [Make sure your fork is up to date before you start committing].

### Adding new examples

- The easiest way to add a new example is to copy **write-to-console** directory and rename it to the example you want.
  Then, update the Cargo.toml file appropriately under the folder you just created.
  You are now good to go.
  Make the necessary changes to the example upon finishing writing code, head to README in the new example's directory, and update it appropriately.


[//]: # (Links)

[BUG_REPORT]:
  https://github.com/umut-sahin/rust-examples/blob/master/.github/ISSUE_TEMPLATE/BUG_REPORT.md
[code of conduct]:
  https://github.com/umut-sahin/rust-examples/blob/master/CODE_OF_CONDUCT.md
[FEATURE_REQUEST]:
  https://github.com/umut-sahin/rust-examples/blob/master/.github/ISSUE_TEMPLATE/FEATURE_REQUEST.md
[GitHub]:
  https://github.com/
[How to Report Bugs Effectively]:
  https://www.chiark.greenend.org.uk/~sgtatham/bugs.html
[Make sure your fork is up to date before you start committing]:
  https://help.github.com/articles/syncing-a-fork/
[Open a new bug report]:
  https://github.com/umut-sahin/rust-examples/issues/new?template=FEATURE_REQUEST.md
[Open a new feature request]:
  https://github.com/umut-sahin/rust-examples/issues/new?template=FEATURE_REQUEST.md
[open a new issue]:
  https://github.com/umut-sahin/rust-examples/issues/new?template=BUG_REPORT.md 
[PULL_REQUEST_TEMPLATE]:
  https://github.com/umut-sahin/rust-examples/blob/master/.github/PULL_REQUEST_TEMPLATE.md
