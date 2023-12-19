# Fix relative links inside index.html
# Website is deployed at https://uncomputable.github.io/simplicity-webide/
# Correct root is /simplicicty-webide/ and not /
sed -i 's|/simplicity-webide|/simplicity-webide/simplicity-webide|g' dist/index.html
