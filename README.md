# Github Analyzer

Github doesn’t make it easy for me to creep on other developer’s commits and coding habits >:(

(No API endpoint to work with commits)

Given a valid Github username (should validate username before performing any expensive operations), PULL A BUNCH OF COMMIT DATA

## TO RUN - :exclamation::exclamation::exclamation:Important:exclamation::exclamation::exclamation:

You need to generate a Personal Access Token with Github and insert it as a single line entry in a file called `token` in root of the repo
The min version of git required is 2.4. This is because the way that your name/email/other identifiers are used to filter the git log changed after that version, and you might as well use the latest version of git instead of apple git or god forbid... - ~Windows~

To verify git installation on MacOS:

- Run `git -v`
- Ensure that the version is 2.4 or above
- if not, you can install git using homebrew: `brew install git`,
- Or, if you already have installed git using brew, use the command `brew upgrade git`

Windows:

- Good luck lol, all I can recommend is installing WSL. The temporary files should be saved to the Temp directory on Windows because I used the TempFile crate, but I can't promise support because I no longer own any Windows machines. I would really appreciate if someone could try this out on Windows and let me know if everything works as it should.

## The motivation:

Github shows users a visual representation of commits, PR’s, code reviews, and any raised issues.

### HOWEVER:

- It’s mainly used for Github clout (more cute than important)
- I want to first write a tool that can pull this data either through the Github API

### OR

- Write my own tool in Rust(!!!) or Node
- Pull all of a user’s repositories (either as several child processes or some combination of parallelly and sequentially (am I gearing up to write job scheduler???))
- Learn about git commits to figure out optimal method to log commits and more importantly commit datetime, but below is the broad plan:
  - machine either starts a new thread or child process to pull the git repo and waits for it to finish.
  - Upon pulling the repository, maybe parse the git log (Gotta figure this part out), and filter out that user’s commits and pull out the timestamp and aggregate number of commits + lines committed maybe?
    - should try to optimize the git log command to display as little text as possible while retaining maximum information
  - After log is fully parsed, delete repository from machine to free up disk space and signal that the machine is ready to start another process (assuming I spin up an army of zombies on AWS) or semaphores/monitors idk

## Icing on the cake

After getting all of this data for a person, maybe I can do some cool analysis on the data?

I really would like to cluster the data after splitting it into distinct “time of day” buckets and cluster on number of lines changed or frequency of commits?

Alternatively, maybe I could use this as a diving board to scrape the same person’s linkedin and use their job placements as a measure of their success hahaha. The idea is to get some insight into what coding habits lead to success and to get this insight with some sort of supervised or reinforcement learning so that a predictive model can be made.

## What this uses:

Rust - Lets get rusty hehehe

The approach is simple:

- Use GitHub API to fetch a user's repositories and scrape the git url from each
- feed fetched git urls into a concurrent job queue that implements work stealing with X num threads
- Clone the repo, parse the filtered git log by user identifier, and delete the repo after saving results to a csv
- **Profit!**

## Development

Github API Token (Personal Access) - HA you thought
If I ever forget, I've saved this with my API key on Notion. Hackers stay away

I’m gonna start by creating a CLI program that takes a username and gets a user’s repositories.

If a user has more than 30 repositories that they’ve committed to, then I have to deal with pagination which is going to be annoying. (Let me think about this later)

Following that, I’m going to create child processes to pull each of these repos and go from there. I can imagine that as input size (num_repositories) goes up, this CLI will be inhibited by the network speed. Not sure what the bounding process will be between processing the git log and cloning the repository. I am also realizing that this will only take into account code that has been merged to the main/master branch of the repository, because I parsing all branches gives rise to the potential for duplicate commits.
