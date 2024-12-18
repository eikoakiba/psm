pub fn banner(VERSION: &f64) {
    println!(
        "MyPass password manager version {} \n\
        A simple and powerfull password manager \n\
        \nUsage: mypass [OPTIONS...] \n\
            \
        \t-a Add a new password \n\
        \t-n Set name for your new password \n\
        \t-d Set description for your new password \n\
        \t-A Add a new password with TUI menu \n\
        \t-l List ALl of your password by their name \n\
        \t-L List all of your password by their name and description and date \n\
        \t-v get the version of this program \n\
        \t-m modify password information",
        VERSION
    )
}
