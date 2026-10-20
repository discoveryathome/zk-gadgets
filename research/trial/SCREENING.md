# Dependency screening

Before building, the SHA-256-verified Discovery@Home helper screened the task folder,
including all fetched Cargo registry sources. It reported no high-severity finding.
Warnings were sudo commands in proc-macro2's GitHub CI configuration, which is not
executed by this trial. Informational items were ordinary Cargo build.rs hooks.
After enabling the merkle_tree feature, the one new dependency (hashbrown 0.14.5)
was screened separately with no findings; its report is in results/.

This is a source-screening result, not a supply-chain audit. Dependencies are locked
by Cargo checksum. Code ran without the agent sandbox, as authorized by the user.
All Cargo downloads and build outputs are under this task folder. No global install,
credential access, GitHub write, repository publication, or service deployment.
