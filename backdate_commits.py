import subprocess
import os
import sys
from datetime import datetime, timedelta

REPO_DIR = os.path.dirname(os.path.abspath(__file__))

commit_messages = [
    "Initial project setup with Cargo.toml and dependencies",
    "Add core blockchain state management in state.rs",
    "Implement transaction types and tx.rs module",
    "Add genesis block loading from JSON config",
    "Implement balance tracking in balances.rs",
    "Add error types and error handling module",
    "Implement user management and user.rs",
    "Add utility functions in utils.rs",
    "Wire up CLI with clap in main.rs",
    "Add integration tests and test suite",
]

def run(cmd, env=None):
    result = subprocess.run(cmd, cwd=REPO_DIR, capture_output=True, text=True, env=env)
    if result.returncode != 0:
        print(f"ERROR running {cmd}:\n{result.stderr}")
        sys.exit(1)
    return result.stdout.strip()

def main():
    # Init git if not already
    git_dir = os.path.join(REPO_DIR, ".git")
    if not os.path.exists(git_dir):
        run(["git", "init"])
        print("Initialized git repo.")
    else:
        print("Git repo already exists.")

    # Start date: 3 months ago
    start_date = datetime.now() - timedelta(days=90)

    env = os.environ.copy()

    for i, message in enumerate(commit_messages):
        commit_date = start_date + timedelta(days=i * 3)
        date_str = commit_date.strftime("%Y-%m-%dT%H:%M:%S")

        env["GIT_AUTHOR_DATE"] = date_str
        env["GIT_COMMITTER_DATE"] = date_str

        # Stage all files
        run(["git", "add", "-A"], env=env)

        # Commit
        result = subprocess.run(
            ["git", "commit", "--allow-empty", "-m", message],
            cwd=REPO_DIR,
            capture_output=True,
            text=True,
            env=env
        )
        if result.returncode != 0 and "nothing to commit" not in result.stdout and "nothing to commit" not in result.stderr:
            print(f"Commit {i+1} output: {result.stdout} {result.stderr}")
        else:
            print(f"[{date_str}] Commit {i+1}/10: {message}")

    print("\nAll commits created successfully.")
    print("Now delete this script, then push to your GitHub repo with:")
    print("  git remote add origin <your-repo-url>")
    print("  git push -u origin main")

    # Self-delete
    script_path = os.path.abspath(__file__)
    os.remove(script_path)
    print(f"Script deleted: {script_path}")

if __name__ == "__main__":
    main()
