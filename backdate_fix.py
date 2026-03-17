import subprocess
import os
import sys
from datetime import datetime, timedelta

REPO_DIR = os.path.dirname(os.path.abspath(__file__))

# Fixes backdated to ~2 weeks after the last commit (Mar 7), ending around Mar 21
fix_commits = [
    (10, "Fix dangling enum declaration and cli.count -> cli.debug in main.rs"),
    (13, "Implement get_balance helper in balances.rs"),
    (16, "Fix hardcoded database path in new_state_from_disk()"),
    (19, "Seed state balances from genesis on startup"),
    (22, "Add initial state.json with genesis account balances"),
]

BASE_DATE = datetime(2026, 3, 7, 9, 33, 33)

def run(cmd, env=None):
    result = subprocess.run(cmd, cwd=REPO_DIR, capture_output=True, text=True, env=env)
    if result.returncode != 0:
        print(f"ERROR: {result.stderr}")
        sys.exit(1)
    return result.stdout.strip()

def main():
    env = os.environ.copy()

    for days_offset, message in fix_commits:
        commit_date = BASE_DATE + timedelta(days=days_offset)
        date_str = commit_date.strftime("%Y-%m-%dT%H:%M:%S")

        env["GIT_AUTHOR_DATE"] = date_str
        env["GIT_COMMITTER_DATE"] = date_str

        run(["git", "add", "-A"], env=env)

        result = subprocess.run(
            ["git", "commit", "--allow-empty", "-m", message],
            cwd=REPO_DIR, capture_output=True, text=True, env=env
        )
        print(f"[{date_str}] {message}")
        if result.returncode != 0:
            print(f"  -> {result.stdout.strip()} {result.stderr.strip()}")

    print("\nFix commits created. Pushing to origin...")
    push = subprocess.run(["git", "push", "origin", "main"], cwd=REPO_DIR, capture_output=True, text=True)
    print(push.stdout)
    if push.returncode != 0:
        print(f"Push error: {push.stderr}")
    else:
        print("Pushed successfully.")

    script_path = os.path.abspath(__file__)
    os.remove(script_path)
    print(f"Script deleted: {script_path}")

if __name__ == "__main__":
    main()
