# Github Analyzer

Github doesn’t make it easy for me to creep on other developer’s commits and coding habits >:(

(No API endpoint to work with commits)

Given a valid Github username (should validate username before performing any expensive operations), PULL A BUNCH OF COMMIT DATA

## TO RUN - :exclamation::exclamation::exclamation:Important:exclamation::exclamation::exclamation:

You need to generate a Personal Access Token with Github and insert it as a single line entry in a file called `token` in root of the repo

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

## Potential Technologies:

- Rust
- Node
- AWS
- Job Scheduler
- Kubernetes
- NGINX (I think, not really sure what it does)

## Development

Github API Token (Personal Access) - HA you thought
If I ever forget, I've saved this with my API key on Notion. Hackers stay away

I’m gonna start by creating a CLI program that takes a username and gets a user’s repositories.

If a user has more than 30 repositories that they’ve committed to, then I have to deal with pagination which is going to be annoying. (Let me think about this later)

Following that, I’m going to create child processes to pull each of these repos and go from there. I can imagine that as input size (num_repositories) goes up, this CLI will be inhibited by the network speed. Not sure what the bounding process will be between processing the git log and cloning the repository. I am also realizing that this will only take into account code that has been merged to the main/master branch of the repository, because I parsing all branches gives rise to the potential for duplicate commits.
