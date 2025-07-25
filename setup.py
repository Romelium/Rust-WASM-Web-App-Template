#!/usr/bin/env python3
import os
import sys
import re
import shutil
import subprocess
import argparse

# --- Constants & Configuration ---
PACKAGE_JSON_PATH = "package.json"
README_PATH = "README.md"
OLD_PROJECT_NAME = "my-project-workspace"
OLD_PROJECT_TITLE = "Rust+WASM Web App Template"

# --- Terminal Colors for Better UI ---
class TColor:
    HEADER = '\033[95m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    GREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

# --- Helper Functions ---

def clear_screen():
    """Clears the terminal screen."""
    os.system('cls' if os.name == 'nt' else 'clear')

def run_subprocess(command, description):
    """Runs a command with robust error handling and clear output."""
    print(f"{TColor.CYAN}-> Running: '{' '.join(command)}' ({description})...{TColor.ENDC}")
    try:
        result = subprocess.run(
            command, check=True, capture_output=True, text=True, encoding='utf-8'
        )
        if result.stdout and result.stdout.strip():
            for line in result.stdout.strip().split('\n'):
                print(f"   {line}")
        print(f"{TColor.GREEN}✓ Success: {description}.{TColor.ENDC}")
        return True
    except FileNotFoundError:
        print(f"{TColor.FAIL}Error: Command '{command[0]}' not found.{TColor.ENDC}", file=sys.stderr)
        print("Please ensure it is installed and in your system's PATH.", file=sys.stderr)
        return False
    except subprocess.CalledProcessError as e:
        print(f"{TColor.FAIL}Error: Failed to {description}.{TColor.ENDC}", file=sys.stderr)
        print(f"Command returned non-zero exit status {e.returncode}.", file=sys.stderr)
        print(f"\n--- stdout ---\n{e.stdout or '[No stdout]'}\n--- stderr ---\n{e.stderr or '[No stderr]'}\n--------------", file=sys.stderr)
        return False

def check_command_exists(command):
    """Checks if a command exists on the system PATH."""
    return shutil.which(command) is not None

def validate_project_name(name):
    """Validates the project name to be a valid npm package name."""
    if not re.match(r'^[a-z0-9]+(-[a-z0-9]+)*$', name):
        print(f"{TColor.FAIL}Error: Invalid project name '{name}'.{TColor.ENDC}", file=sys.stderr)
        print("Name must be all lowercase, start/end with a letter/number, and can contain hyphens.", file=sys.stderr)
        return None
    return name

def to_title_case(kebab_case_str):
    """Converts a kebab-case string to a Title Case string."""
    return ' '.join(word.capitalize() for word in kebab_case_str.split('-'))

def replace_in_file(file_path, old_str, new_str):
    """Safely replaces a string in a file."""
    if not os.path.exists(file_path):
        print(f"{TColor.WARNING}Warning: File '{file_path}' not found. Skipping.{TColor.ENDC}")
        return False
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        if old_str not in content:
            return True # Nothing to do, so it's a success
        new_content = content.replace(old_str, new_str)
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        return True
    except Exception as e:
        print(f"{TColor.FAIL}Error: Failed to update '{file_path}': {e}{TColor.ENDC}", file=sys.stderr)
        return False

def remove_setup_section_from_readme():
    """Removes the setup instructions from the README file."""
    print(f"{TColor.CYAN}Cleaning up README.md...{TColor.ENDC}")
    if not os.path.exists(README_PATH):
        return True # File not found, nothing to do
    try:
        with open(README_PATH, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Use a regex to find and remove the block including the markers and any trailing newline
        pattern = re.compile(r'<!-- SETUP_START -->.*?<!-- SETUP_END -->\n?', re.DOTALL)
        new_content = re.sub(pattern, '', content)

        with open(README_PATH, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"{TColor.GREEN}✓ README.md cleaned.{TColor.ENDC}")
        return True
    except Exception as e:
        print(f"{TColor.FAIL}Error: Failed to clean up README.md: {e}{TColor.ENDC}", file=sys.stderr)
        return False

def delete_script():
    """Deletes the setup script itself after a successful run."""
    try:
        print(f"\n{TColor.CYAN}Self-destructing setup script...{TColor.ENDC}")
        os.remove(__file__)
        print(f"{TColor.GREEN}✓ Script deleted.{TColor.ENDC}")
    except OSError as e:
        print(f"{TColor.WARNING}Warning: Could not delete setup.py: {e}{TColor.ENDC}", file=sys.stderr)
        print("Please delete it manually.", file=sys.stderr)

# --- Core Action Functions ---

def action_ensure_pnpm():
    """Checks for pnpm and installs it via npm if not found."""
    if check_command_exists("pnpm"):
        return True
    
    print(f"{TColor.WARNING}pnpm not found. Attempting to install globally via npm...{TColor.ENDC}")
    if not check_command_exists("npm"):
        print(f"{TColor.FAIL}Error: 'npm' is required to install 'pnpm', but it was not found.{TColor.ENDC}")
        return False
    return run_subprocess(["npm", "install", "-g", "pnpm"], "install pnpm")

# --- Main Logic ---

def get_project_name_interactively():
    """Guides the user to enter a valid project name."""
    clear_screen()
    print(f"{TColor.HEADER}{TColor.BOLD}--- Step 1: Set Project Name ---{TColor.ENDC}")
    print("Please provide a new name for your project.")
    print("It must be all lowercase and can contain hyphens (e.g., 'my-cool-app').")
    
    while True:
        try:
            name_input = input(f"\n{TColor.BOLD}New project name: {TColor.ENDC}").strip()
            if not name_input:
                print(f"{TColor.WARNING}Project name cannot be empty.{TColor.ENDC}")
                continue
            
            validated_name = validate_project_name(name_input)
            if validated_name:
                return validated_name
        except (KeyboardInterrupt, EOFError):
            print("\n\nSetup cancelled by user.")
            sys.exit(0)

def confirm_and_run_actions_interactively(project_name, has_cargo, has_pnpm, has_npm):
    """Shows a confirmation menu where users can toggle actions and then execute."""
    actions = {
        'rename': True,
        'cargo_upgrade': True,
        'pnpm_install': True,
    }

    while True:
        clear_screen()
        print(f"{TColor.HEADER}{TColor.BOLD}--- Step 2: Confirm Actions ---{TColor.ENDC}")
        print(f"Project will be named: {TColor.GREEN}{project_name}{TColor.ENDC}")

        print(f"\n{TColor.BOLD}The following actions will be performed:{TColor.ENDC}")
        
        rename_check = f"[{'x' if actions['rename'] else ' '}]"
        cargo_check = f"[{'x' if actions['cargo_upgrade'] else ' '}]"
        pnpm_check = f"[{'x' if actions['pnpm_install'] else ' '}]"
        
        print(f"   {TColor.CYAN}(t)oggle{TColor.ENDC} {rename_check} Rename project files")
        
        cargo_status = "" if has_cargo else f" {TColor.WARNING}('cargo' not found){TColor.ENDC}"
        print(f"   {TColor.CYAN}(u)oggle{TColor.ENDC} {cargo_check} Upgrade Rust dependencies{cargo_status}")
        
        pnpm_status = ""
        if not has_pnpm and not has_npm:
            pnpm_status = f" {TColor.WARNING}('pnpm' and 'npm' not found){TColor.ENDC}"
        elif not has_pnpm:
            pnpm_status = f" {TColor.CYAN}(will be installed via npm){TColor.ENDC}"
        print(f"   {TColor.CYAN}(i)oggle{TColor.ENDC} {pnpm_check} Install Node.js dependencies{pnpm_status}")

        print(f"\n{TColor.BOLD}Menu:{TColor.ENDC}")
        print(f"  (t, u, i) Toggle an action")
        print(f"  ({TColor.GREEN}g{TColor.ENDC}) Go! Run selected actions")
        print(f"  (b) Back to rename project")
        print(f"  (q) Quit")

        try:
            choice = input("\nEnter your choice: ").lower().strip()

            if choice == 't': actions['rename'] = not actions['rename']
            elif choice == 'u': actions['cargo_upgrade'] = not actions['cargo_upgrade']
            elif choice == 'i': actions['pnpm_install'] = not actions['pnpm_install']
            elif choice == 'b': return False # Signal to go back
            elif choice == 'g':
                config = {'new_project_name': project_name, 'actions': actions}
                run_selected_actions(config, has_cargo, has_pnpm, has_npm, is_interactive=True)
                return True # Signal that we are done
            elif choice == 'q':
                print("Setup cancelled.")
                sys.exit(0)
        except (KeyboardInterrupt, EOFError):
            print("\n\nSetup cancelled.")
            sys.exit(0)

def run_interactive_mode():
    """Runs the script in a guided, step-by-step interactive mode."""
    clear_screen()
    print(f"{TColor.HEADER}{TColor.BOLD}--- Interactive Project Setup ---{TColor.ENDC}")
    print("Welcome! This script will help you personalize this template.")

    print("\nChecking for required tools...")
    has_cargo = check_command_exists("cargo")
    has_pnpm = check_command_exists("pnpm")
    has_npm = False if has_pnpm else check_command_exists("npm") # Only check for npm if pnpm is missing

    print(f"Rust 'cargo' command:   {'Found' if has_cargo else f'{TColor.WARNING}Not Found{TColor.ENDC}'}")
    if has_pnpm:
        print(f"Node 'pnpm' command:    {TColor.GREEN}Found{TColor.ENDC}")
    else:
        print(f"Node 'pnpm' command:    {TColor.WARNING}Not Found{TColor.ENDC}")
        if has_npm:
            print(f"  ↳ 'npm' fallback:     {TColor.GREEN}Found (can be used to install pnpm){TColor.ENDC}")
        else:
            print(f"  ↳ 'npm' fallback:     {TColor.FAIL}Not Found (required to install pnpm){TColor.ENDC}")
    
    input("\nPress Enter to continue...")

    while True:
        project_name = get_project_name_interactively()
        finished = confirm_and_run_actions_interactively(project_name, has_cargo, has_pnpm, has_npm)
        if finished:
            break # Exit the loop and the script

def run_non_interactive_mode(project_name, skip_confirmation):
    """Runs all setup steps automatically. Ideal for CI."""
    print(f"{TColor.HEADER}--- Automated Project Setup ---{TColor.ENDC}")
    print(f"Project will be renamed to: {TColor.CYAN}{project_name}{TColor.ENDC}")
    
    if not skip_confirmation:
        try:
            confirm = input("This will modify project files and install dependencies. Continue? (y/n): ").lower()
            if confirm not in ['y', 'yes']:
                print("Setup cancelled.")
                sys.exit(0)
        except (KeyboardInterrupt, EOFError):
            print("\nSetup cancelled.")
            sys.exit(0)

    has_cargo = check_command_exists("cargo")
    has_pnpm = check_command_exists("pnpm")
    has_npm = False if has_pnpm else check_command_exists("npm")
    config = {
        'new_project_name': project_name,
        'actions': {'rename': True, 'cargo_upgrade': True, 'pnpm_install': True}
    }
    run_selected_actions(config, has_cargo, has_pnpm, has_npm, is_interactive=False)

def run_selected_actions(config, has_cargo, has_pnpm, has_npm, is_interactive):
    """Executes the actions based on the config dictionary."""
    clear_screen()
    print(f"{TColor.HEADER}{TColor.BOLD}--- Executing Actions ---{TColor.ENDC}\n")
    actions = config['actions']
    
    # --- Validation Step ---
    if actions['cargo_upgrade'] and not has_cargo:
        print(f"{TColor.FAIL}Error: 'Upgrade Rust dependencies' is selected, but 'cargo' was not found.{TColor.ENDC}")
        if is_interactive: input("Press Enter to return to the menu...")
        return
    if actions['pnpm_install'] and not has_pnpm and not has_npm:
        print(f"{TColor.FAIL}Error: 'pnpm install' is selected, but 'pnpm' is not installed and 'npm' is not available to install it.{TColor.ENDC}")
        if is_interactive: input("Press Enter to return to the menu...")
        return

    all_success = True

    # --- Execution Step ---
    if actions['rename']:
        print(f"{TColor.BOLD}1. Renaming Project...{TColor.ENDC}")
        name = config['new_project_name']
        title = to_title_case(name)
        if not replace_in_file(PACKAGE_JSON_PATH, f'"name": "{OLD_PROJECT_NAME}"', f'"name": "{name}"'): all_success = False
        if not replace_in_file(README_PATH, f'# {OLD_PROJECT_TITLE}', f'# {title}'): all_success = False
        if all_success: print(f"{TColor.GREEN}✓ Project renamed to '{name}'.{TColor.ENDC}\n")

    if all_success and actions['cargo_upgrade']:
        print(f"{TColor.BOLD}2. Upgrading Rust Dependencies...{TColor.ENDC}")
        if not run_subprocess(["cargo", "upgrade"], "update Rust dependencies"): all_success = False
        print("")

    if all_success and actions['pnpm_install']:
        print(f"{TColor.BOLD}3. Installing Node.js Dependencies...{TColor.ENDC}")
        if not action_ensure_pnpm():
            all_success = False
        else:
            if not run_subprocess(["pnpm", "install"], "install Node.js dependencies"): all_success = False
        print("")

    # --- Finalization Step ---
    if all_success:
        print(f"{TColor.GREEN}{TColor.BOLD}🚀 All selected actions completed successfully!{TColor.ENDC}")
        if not remove_setup_section_from_readme():
            print(f"{TColor.WARNING}Could not clean the README.md file. Please remove the 'First-Time Setup' section manually.{TColor.ENDC}")
        delete_script()
        print("\nNext steps:")
        print("1. Commit the changes to your git repository.")
        print("2. Start the development server with: cargo run --bin cli -- dev")
    else:
        print(f"\n{TColor.FAIL}{TColor.BOLD}Setup finished with errors. Please review the output above.{TColor.ENDC}")
        print("The setup script has not been deleted so you can run it again.")
    
    if is_interactive:
        input("\nPress Enter to exit...")

def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="A setup script to rename the project and install dependencies.",
        formatter_class=argparse.RawTextHelpFormatter
    )
    parser.add_argument(
        "project_name",
        nargs='?', # Make the argument optional
        default=None,
        help="The new name for the project (e.g., 'my-cool-app').\nIf omitted, runs in interactive mode."
    )
    parser.add_argument(
        "-y", "--yes",
        action="store_true",
        help="Skip confirmation prompts in non-interactive mode."
    )
    args = parser.parse_args()

    if args.project_name:
        validated_name = validate_project_name(args.project_name)
        if not validated_name:
            sys.exit(1)
        run_non_interactive_mode(validated_name, args.yes)
    else:
        run_interactive_mode()

if __name__ == "__main__":
    main()