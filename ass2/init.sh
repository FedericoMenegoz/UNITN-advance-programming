#!/bin/bash

# Check if the correct number of arguments is provided
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <number_of_files>"
    exit 1
fi

# Get the number of files from the command-line argument
num_files=$1


# Function to prompt the user and get a yes/no response
function prompt_user() {
    while true; do
        read -p "$1 (y/n): " yn
        case $yn in
            [Yy]* ) return 0;;  # User wants to continue
            [Nn]* ) return 1;;  # User does not want to continue
            * ) echo "Please answer yes or no.";;
        esac
    done
}

# Check if files already exist
for ((i = 1; i <= num_files; i++)); do
    filename="src/es${i}.rs"
    if [ -e "$filename" ]; then
        if prompt_user "File $filename already exists. Do you want to overwrite it?"; then
            rm "$filename"
        else
            echo "Script aborted by user."
            exit 0
        fi
    fi
done

# Create es<1>.rs to es<n>.rs files with templates
for ((i = 1; i <= num_files; i++)); do
    filename="src/es${i}.rs"
    echo "pub fn es${i}() {" >> "$filename"
    # Add more template content if needed
    echo "}" >> "$filename"
done

# Write on main.rs
main_file="src/main.rs"
echo "pub mod es1;" > "$main_file"
for ((i = 2; i <= num_files; i++)); do
    echo "pub mod es${i};" >> "$main_file"
done

echo -e "\nfn main() {" >> "$main_file"
for ((i = 1; i <= num_files; i++)); do
    echo "    println!(\"\\n********** *********** **********\");" >> "$main_file"
    echo "    println!(\"           EXERCIZE ${i}           \");" >> "$main_file"
    echo "    println!(\"********** *********** **********\\n\");" >> "$main_file"
    echo "    es${i}::es${i}();" >> "$main_file"
done
echo "}" >> "$main_file"

echo "Created template for ${num_files} exercizes!"
