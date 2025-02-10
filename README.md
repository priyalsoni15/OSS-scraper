Open Source Sustainability Scraper for Github projects
========================

Overview
--------
This RUST Tool is a robust data scraper designed to extract valuable information from a local Git repository. It provides insights into:
- Commit history and metadata
- Number of lines changed
- Developer contributions
- Files added or modified
- Mailing box emails (if applicable)
- Raw data for generating social and technical networks

Note: Constructing a social network requires analysing communications between developers (which could be in the form of GitHub issues or Mail archives (if applicable).
Constructing a technical network required analysing the commits made by a developer to the Github repo (files changed, files added, activity ratio, etc).

------------------------

Prerequisites
-------------
Before running this tool, ensure you have:
1. A local GitHub project downloaded into the specified path.
2. Rust and Cargo installed on your system. You can follow the official Rust installation guide at:  
   https://www.rust-lang.org/tools/install

------------------------

Commands
--------

Please note that these commands and files have been set according to the folder structure requirements in the DECAL Lab server and may change when used locally. If scraping 50+ projects, it is recommended to run the tool on a cloud or dedicated server.

**Setting Up the Environment**

Run the following commands to prepare the environment and build the tool:

    cargo update
    cargo clean
    cargo build
    cargo fix --bin "miner"

------------------------

**Running the Tool**

### Fetching issues from a GitHub repository

    ./target/debug/miner --fetch-github-issues --github-url=https://github.com/apache/hunter.git --github-output-folder=output

_(Github URL is the project to analyze, and the output folder stores the CSV analysis)_

### Fetching issues sorted by developers in a GitHub repository (separate csvs for each developer)

    ./target/debug/miner --fetch-github-issues  --issue-stats-grouped --github-url=https://github.com/apache/hunter.git --github-output-folder=output

### Collecting commit details (file changes, authors, hashes, etc.)

    ./target/debug/miner --skip-emails --skip-sokrates --ignore-start-end-date --commit-devs-files --time-window=30 --threads=2 --output-folder=output --git-folder=input

### Fetching commit details using online versioning

    ./target/debug/miner --skip-emails --ignore-start-end-date --commit-devs-files --time-window=30 --threads=2 --output-folder=output --git-online-url=https://github.com/apache/hunter.git --online-start-date=2020-11-14 --online-end-date=2025-02-05 --online-status=""

### Fetching commit details with online versioning (dynamic start and end dates)

    ./target/debug/miner --commit-devs-files --time-window=30 --threads=2 --output-folder=output --git-online-url=https://github.com/apache/hunter.git

### Fetching commit details using GraphQL

    ./target/debug/miner --commit-devs-files --git-online-url=https://github.com/apache/hunter.git --commit-graphql --online-start-date=2020-11-14 --online-end-date=2025-02-05 --online-status="" --threads=2 --output-folder=output

### Developer commit metrics (All months)

    ./target/debug/miner --skip-emails --skip-sokrates --commit-devs-files --ignore-start-end-date --commit-devs-files --dev-stats-grouped --time-window=30 --threads=2 --output-folder=output --git-folder=input

### Full analysis (excluding email analysis, for local Git repositories)

    ./target/debug/miner --skip-emails --ignore-start-end-date --force-full-analysis --time-window=30 --threads=2 --output-folder=output --git-folder=input

### Downloading emails

    ./target/debug/miner --download-emails --restrict-languages --ignore-start-end-date --output-folder=output/ --git-folder=input/ --project=aCute

------------------------

### Notes
-----
1. Ensure all paths are correctly set before executing the commands.
2. Adjust parameters (e.g., `time-window`, `threads`) as required for your analysis needs.
------------------------

### Contributing
Contributions are welcome! However, please do ensure that you are not committing directly to this repo. However, absolutely feel free to fork it and open a Pull Request for minor fixes. For major changes, I'd recommend opening an issue first to discuss your proposal, and we'll take it from there.

### Contact
In case of any questions, feel free to reach out to priyal15.soni@gmail.com or pdsoni@ucdavis.edu.

### License
This project is licensed under the Apache License 2.0.

------------------------

