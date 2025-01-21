struct ProjectStats {}

impl ProjectStats {
    pub fn checkout_master() -> Result<(), Err> {}
    pub fn find_months() -> Result<(), Err> {}
    pub fn analyze_commits() -> Result<(), Err> {}
    pub fn analyze_emails() -> Resulst<(), Err> {}
    pub fn compute_stats() -> Result<(), Err> {}
}

fn test() {
    let stats = ProjectStats()
        .checkout_master()?
        .find_months()?
        .analyze_commits()?
        .analyze_emails()?
        .compute_stats()?;
}
