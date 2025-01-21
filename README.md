
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

This tool is particularly useful for analyzing code quality, developer activities, and collaboration patterns.

------------------------

Prerequisites
-------------
Before running this tool, ensure you have:
1. A local GitHub project downloaded into the specified path (referenced as `/mnt/data1/eclipse-data/git` in the examples below).
2. Rust and Cargo installed on your system. You can follow the official Rust installation guide at:  
   https://www.rust-lang.org/tools/install

------------------------

Commands
--------

Please note that these commands and files have been set according to the folder structure requirements in the DECAL Lab server, and is subjected to change if used locally. Although it is recommended that if there is scraping to be done for 50+ projects, ensure that you run it on cloud or another server, instead of your own system.

**Setting Up the Environment**

Run the following commands to prepare the environment and build the tool:

    # Navigate to the Rust code directory
    cd /mnt/data1/asf-code-quality/volunteer-energy/rust-code

    # Update dependencies, clean old builds, and fix issues
    sudo cargo update
    sudo cargo clean
    sudo cargo fix --bin "miner"

    # Build the project
    sudo cargo build

    # Copy the log4rs.yaml configuration file to the target directory
    cp /mnt/data1/asf-code-quality/volunteer-energy/rust-code/log4rs.yaml /mnt/data1/asf-code-quality/volunteer-energy/rust-code/target/debug/

------------------------

**Creating Output Folder**

Before running the tool, create an output folder to store the results:

    mkdir -p /mnt/data1/eclipse-data/eclipse-analysis-data

------------------------

**Running the Tool**

**Basic Run**

    sudo ./target/debug/miner --skip-emails --skip-sokrates --restrict-languages --ignore-start-end-date --output-folder=/mnt/data1/eclipse-data/eclipse-analysis-data --time-window=30 --threads=2 --git-folder=/mnt/data1/eclipse-data/git

**Project-Specific Run**

    sudo ./target/debug/miner --skip-emails --skip-sokrates --restrict-languages --ignore-start-end-date --output-folder=/mnt/data1/pdsoni/rust-test/output/ --time-window=30 --threads=2 --git-folder=/mnt/data1/pdsoni/rust-test/ --project=aCute

**Commit, Developers, and Files Analysis**

    sudo ./target/debug/miner --commit-devs-files --ignore-commit-message --skip-emails --skip-sokrates --restrict-languages --ignore-start-end-date --output-folder=/mnt/data1/pdsoni/rust-test/output/ --time-window=30 --threads=2 --git-folder=/mnt/data1/pdsoni/rust-test/ --project=aCute

**Full Analysis for a Specific Project**

    sudo ./target/debug/miner --full-analysis --ignore-start-end-date --output-folder=/mnt/data1/pdsoni/rust-test/output/ --time-window=30 --threads=4 --git-folder=/mnt/data1/pdsoni/rust-test/ --project=aCute

**Downloading Data from Mailboxes**

    sudo ./target/debug/miner --download-emails --restrict-languages --ignore-start-end-date --output-folder=/mnt/data1/pdsoni/rust-test/output/ --git-folder=/mnt/data1/pdsoni/rust-test/ --project=aCute

------------------------

**Verifying Output**

Navigate to the output directory to verify the generated data:

    cd /mnt/data1/pdsoni/rust-test/output/

------------------------

### Notes
-----
1. Ensure all paths are correctly set before executing the commands.
2. Use `sudo` only if necessary based on your system's configuration.
3. Adjust parameters (e.g., `time-window`, `threads`) as required for your analysis needs.

------------------------

### Contributing
Contributions are welcome! However, do ensure that you are not committing directly to this repo. Do feel free to fork this though, and open a Pull Request for a minor fix or otherwise. For major changes, please open an issue first to discuss what you'd like to change. And we can work on integrating it!

### Contact
In case of any questions, feel free to reach out to priyal15.soni@gmail.com or pdsoni@ucdavis.edu.

### License
This project is licensed under the Apache License 2.0.

------------------------

