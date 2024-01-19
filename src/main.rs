
use std::{env, process, fs, path};
use std::path::Path;
use std::os::unix::fs::PermissionsExt;


fn pwd() {
    let path = env::current_dir();
    println!("{}", path.unwrap().to_string_lossy());
}

fn echo(args: & Vec<String>) -> i32 {

    if args.len() < 3 {
        println!("{}", 246);
        process::exit(246);
    }

    let mut result = String::new();
     let mut newline = 1;     
    if args[2] == "-n"{
        newline = 0;
    }

    if newline == 1{
        for arg in &args[2..]{
            result.push_str(arg);
            result.push(' ');
            
        }
    }
    else{
        for arg in &args[3..]{
            result.push_str(arg);
            result.push(' ');
        }
    }
    result.pop();
    if newline == 1{
        println!("{}", result);
    }
    else{
        print!("{}", result);
    }
    0
}
 
fn cat(args: &Vec<String>) -> i32 {

    if args.len() < 3{
        println!("{}", 226);
        process::exit(226);
    }

    for arg in &args[2..] {
        if let Ok(result) = fs::read_to_string(&arg) {
            print!("{}", result);
        } else {
            println!("{}", 236);
            process::exit(236);
        }
    }
    0
}

fn mkdir(args: &Vec<String>) -> i32 {
    for arg in &args[2..]{
       if !fs::metadata(&arg).is_ok(){
            if let Err(_) = fs::create_dir(&arg){
                println!("{}", 226);
                process::exit(226);
            }
           
        }
    }
    0
}

fn mv(args: &Vec<String>) -> i32 {
    if args.len() < 4{
        println!("{}", 216);
        process::exit(216);
    }
    if let Err(_) = fs::rename(&args[2], &args[3]){
        println!("{}", 216);
        process::exit(216);
    }
    0
}

fn touch(args: &Vec<String>) -> i32{
    if args.len() < 3{
        println!("{}", 156);
        process::exit(156);
    }
    let mut param = 0;
    let mut _access = 0;
    let mut _modif = 0;
    let mut create = 0;
    
    for arg in &args[2..]{
        if arg == "-a"{
            _access = 1;
            param += 1 ;
        }
        if arg == "-m"{
            _modif = 1;
            param += 1;
        }
        if arg == "-c" || arg == "--no-create"{
            create = 1;
            param += 1;
        }
    }

    let i = param + 2;
    for arg in &args[i..]{
        let path = path::Path::new(&arg);
        if create == 0 && !fs::metadata(path).is_ok(){
            if let Err(_) = fs::File::create(path){
                println!("{}", 156);
                process::exit(156);
            }
        }
        if create == 0 {
            if let Err(_) = fs::remove_file(path){
                println!("{}", 156);
                process::exit(156);
            }
            if let Err(_) = fs::File::create(path){
                println!("{}", 156);
                process::exit(156);
            }
        }
    }


    0

}


fn ln(args: &Vec<String>) -> i32{

    if args.len() < 3{
        println!("{}", 206);
        process::exit(206);
    }
    let mut param = 0;
    if args[2] == "-s" || args[2] == "--symbolic"{
        param += 1;
    }
    let mut simbolic = 0;

    if param == 1{
        simbolic = 1;
    }
    if param == 0 && args.len() > 4{
        println!("Invalid command");
        process::exit(-1);
    }
    

    
    if simbolic == 1 {
        let source = &args[3];
        let link_name = &args[4];
        if let Err(_) = std::os::unix::fs::symlink(source, link_name){
            println!("{}", 206);
            process::exit(206);
        }
    }else{
        let source = &args[2];
        let link_name = &args[3];
        if let Err(_) = fs::hard_link(source, link_name){
            println!("{}", 206);
            process::exit(206);
        }
    }
    0
}


fn rmdir(args: & Vec<String>) ->i32 {
    if args.len() < 2 {
        println!("{}", 196);
        process::exit(196);
    }
    for arg in &args[2..]{
        if let Err(_) = fs::remove_dir(&arg){
            println!("{}", 196);
            process::exit(196);
        }
    }

    0
}

fn rm(args: &Vec<String>) -> i32{
    if args.len() < 3{
        println!("{}", 186);
        process::exit(186);
    }
    let mut dir = 0;
    let mut recursive = 0;
    let mut dir_error = 0;
    let mut nr_param = 1;
    for arg in &args[2..]{
        if arg == "-r" || arg == "-R" || arg == "--recursive"{
            recursive = 1;
            nr_param += 1;
        }
        if arg == "-d" || arg == "--dir"{
            dir = 1;
            nr_param += 1;
        }
    }
    if nr_param == (args.len() - 1){
        println!("Invalid command");
        process::exit(-1);
    }
    if recursive == 1 && dir == 1 {
        for arg in &args[4..] {
            let path = path::Path::new(&arg);
            if path.exists() {
                if path.is_dir(){
                    if let Err(_) = fs::remove_dir_all(&arg) {
                        println!("{}", 186);
                        process::exit(186);
                            
                    }
                }else if path.is_file(){
                    if let Err(_) = fs::remove_file(&arg) {
                        println!("{}", 186);
                        process::exit(186);
                    }
                }
            }
        }
    }
    
        if dir == 1 && recursive == 0{
            for arg in &args[3..]{
                let path = path::Path::new(&arg);
                if path.exists(){
                
                    if let Ok(result) = fs::metadata(&arg){
                        if !result.is_dir(){
                            println!("{}", 186);
                            process::exit(186);
                        }
                    }
                    if let Err(_) = fs::remove_dir(&arg){
                        println!("{}", 186);
                        process::exit(186);
                    }
                }else{
                    println!("{}", 186);
                    process::exit(186);
                }
            }
        }
            if recursive == 1 && dir == 0{
                for arg in &args[3..]{
                    let path = path::Path::new(&arg);
                    if path.exists(){
                        if let Ok(result) = fs::metadata(&arg){
                            if result.is_file(){
                                if let Err(_) = fs::remove_file(&arg){
                                    println!("{}", 186);
                                    process::exit(186);
                                }
                            }else{
                                if result.is_dir(){
                                    if let Err(_) = fs::remove_dir_all(&arg){
                                        println!("{}", 186);
                                        process::exit(186);
                                    }
                                }
                            }
                        }
                    }else{
                        println!("{}", 186);
                        process::exit(186);
                    }
                }
            }
            if recursive == 0 && dir == 0{
                for arg in &args[2..]{
                    let path = path::Path::new(&arg);
                    if path.exists(){
                        if let Ok(result) = fs::metadata(&arg){
                            if result.is_file(){
                                if let Err(_) = fs::remove_file(&arg){
                                    println!("{}", 186);
                                    process::exit(186);
                                }
                            }else{
                                if result.is_dir(){
                                        dir_error = 1;
                                }
                            }  
                        }
                    }else{
                        println!("{}", 186);
                        process::exit(186);
                    }
                }
            }

    
        
    
    if dir_error == 1{
        println!("{}", 186);
        process::exit(186);
    }
    0
}



fn cp(args: &Vec<String>) ->i32 {
    if args.len() < 3 {
        println!("{}", 166);
        process::exit(166);
    }
    let mut recursive = 0;
    if args[2] == "-r" || args[2] == "-R" || args[2] == "--recursive"{
        recursive = 1;
    }
    if recursive == 0{
        let sursa = &args[2];
        let path_sursa = path::Path::new(&sursa);
            let destinatie = &args[3];
            let path_dest = path::Path::new(&destinatie);
            if path_dest.exists(){
                if path_sursa.is_file(){
                    if path_dest.is_dir(){
                        let dest_path = path_dest.join(path_sursa.file_name().unwrap());
                        if let Err(_) = fs::copy(path_sursa, dest_path){
                            println!("{}", 166);
                            process::exit(166);
                        }
                    }
                }
            }else{
                if path_sursa.is_file(){
                    if let Err(_) = fs::copy(path_sursa, path_dest){
                        println!("{}", 166);
                        process::exit(166);
                    }
                }else{
                    println!("{}", 166);
                    process::exit(166);
                }

            }
    }
    if recursive == 1{
        let sursa = &args[3];
        let destinatie = &args[4];
        let path_dest = path::Path::new(destinatie);
        let path_sursa = path::Path::new(sursa);
        if path_sursa.is_dir(){
            let dest_path = path_dest.join(path_sursa.file_name().unwrap());
            copy_recursive(path_sursa, &dest_path);
            copy_recursive(path_sursa, path_dest);
        }else if path_sursa.is_file(){
            let dest_path = path_dest.join(path_sursa.file_name().unwrap());
            if let Err(_) = fs::copy(path_sursa, dest_path){
                println!("{}", 166);
                process::exit(166);
            }
        }
    }
    
    0
}

fn copy_recursive(path_sursa: &Path, path_dest: &Path){
   if !path_dest.exists(){
        if let Err(_) = fs::create_dir_all(path_dest){
            println!("{}", 166);
            process::exit(166);
        }
   }
   for entry in fs::read_dir(path_sursa).unwrap(){
       let entry = entry.unwrap();
       let path = entry.path();
       let dest_path = path_dest.join(path.file_name().unwrap());
       if path.is_dir(){
           copy_recursive(&path, &dest_path);
       }else{
           if let Err(_) = fs::copy(path, dest_path){
               println!("{}", 166);
               process::exit(166);
           }
       }
   }

}

fn ls(args: & Vec<String>) -> i32{
    if args.len() < 2{ 
        println!("{}", 176);
        process::exit(176);
    }
    let mut param = 0;
    let mut all =0;
    let mut recursive = 0;  
    let mut dir = 0;
    for arg in &args[2..]{
        if arg == "-a" || arg == "--all"{
            all = 1;
            param += 1; 
        }
        if arg == "-R" || arg == "-r" || arg == "--recursive"{
            recursive = 1;
            param += 1;
        }
   }
    if param + 2 < args.len(){
        dir = 1;
    }



    if all == 1 && recursive == 0 && dir == 0{
        let current_dir = env::current_dir();
        if let Ok(result) = current_dir{
            if let Ok(entries) = fs::read_dir(result){
                for entry in entries{
                    if let Ok(entry) = entry{
                        if let Ok(file_name) = entry.file_name().into_string(){
                            println!("{}", file_name);
                        }
                    }
                }
            }
        }else{
            println!("{}", 176);
            process::exit(176);
        }
    }

    if all == 0 && recursive == 0 && dir == 0{
        let current_dir = env::current_dir();
        if let Ok(result) = current_dir{
            if let Ok(entries) = fs::read_dir(result){
                for entry in entries{
                    if let Ok(entry) = entry{
                        if let Ok(file_name) = entry.file_name().into_string(){
                            if file_name.starts_with('.') == false{
                                println!("{}", file_name);
                            }
                        }
                    }else{
                        println!("{}", 176);
                        process::exit(176);
                        
                    }
                }
            }
        }else{
            println!("{}", 176);
            process::exit(176);
        }
    }



    if all == 1 && recursive == 0 && dir == 1{
      let directory = &args[3];
      let path = path::Path::new(&directory);
        if let Ok(entries) = fs::read_dir(path){
            for entry in entries{
                if let Ok(entry) = entry{
                    if let Ok(file_name) = entry.file_name().into_string(){
                        println!("{}", file_name);
                    }
                }
            }
            println!(".");
            println!("..");
        }else{
            println!("{}", 176);
            process::exit(176);
        }
    }

    if all == 0 && recursive == 0 && dir == 1{
        let current_dir = &args[2];
        let path = path::Path::new(&current_dir);
        if path.exists(){
            if let Ok(meta) = fs::metadata(current_dir){
                if meta.is_dir(){
                    if let Ok(entries) = fs::read_dir(current_dir){
                        for entry in entries{
                            if let Ok(entry) = entry{
                                if let Ok(file_name) = entry.file_name().into_string(){
                                    if file_name.starts_with('.') == false{
                                        println!("{}", file_name);
                                    }
                                }
                            }else{
                                println!("{}", 176);
                                process::exit(176);
                                
                            }
                        }
                    }else{
                        println!("{}", 176);
                        process::exit(176);
                    }
                }else if meta.is_file(){
                    println!("{}", current_dir);
                }
            }
        }else{
            println!("{}", 176);
            process::exit(176);
        }
    }
    
    if all == 0 && recursive == 1 && dir == 1{
        let current_dir = &args[3];
        let path = path::Path::new(&current_dir);
        recursive_ln(path);
    }
    if all == 1 && recursive == 1 && dir == 1{
        let current_dir = &args[4];
        let path = path::Path::new(&current_dir);
        println!(".");
        println!("..");
        recursive_ln_all(path);
    }
    
    
    0
}


fn recursive_ln(current_dir: &Path) {
    if let Ok(meta) = fs::metadata(current_dir){
        if meta.is_dir(){
            println!("{}:", current_dir.to_str().unwrap());
        }
    }
    if let Ok(entries) = fs::read_dir(current_dir){
        for entry in entries{
            if let Ok(entry) = entry{
                if let Ok(meta) = fs::metadata(&entry.path()){
                    if meta.is_dir(){
                        if let Ok(path2) = entry.path().into_os_string().into_string(){
                            if path2.starts_with('.') == false{
                                if let Some(index) = path2.rfind('/') {
                                    let path3 = &path2[index+1..];
                                    println!("{}", &path3);
                                    recursive_ln(&entry.path());
                                }
                            }
                        }
                        
                    }else{
                        if let Ok(path2) = entry.path().into_os_string().into_string(){
                            if path2.starts_with('.') == false{
                               if let Some(index) = path2.rfind('/') {
                                    let path3 = &path2[index+1..];
                                    println!("{}", &path3);
                                }
                            }
                        }
                    }
                }else{
                    println!("{}", 176);
                    process::exit(176);
                }
            }
        }
    }else{
        println!("{}", 176);
        process::exit(176);
    }
    
}


fn recursive_ln_all(current_dir: &Path) {
    if let Ok(meta) = fs::metadata(current_dir){
        if meta.is_dir(){
            println!("{}:", current_dir.to_str().unwrap());
        }
    }
    if let Ok(entries) = fs::read_dir(current_dir){
        for entry in entries{
            if let Ok(entry) = entry{
                if let Ok(meta) = fs::metadata(&entry.path()){
                    if meta.is_dir(){
                        if let Ok(path2) = entry.path().into_os_string().into_string(){
                            if let Some(index) = path2.rfind('/') {
                                let path3 = &path2[index+1..];
                                println!("{}", path3);
                                recursive_ln_all(&entry.path());
                            }
                        }
                        
                    }else{
                        if let Ok(path2) = entry.path().into_os_string().into_string(){
                            if let Some(index) = path2.rfind('/') {
                                let path3 = &path2[index+1..];
                                println!("{}", path3);
                            }
                        }
                    }
                    if meta.is_dir(){
                        let mut path4 = entry.path().into_os_string().into_string().unwrap();
                        path4.push_str("/");
                        path4.push_str(".");
                        if let Some(index) = path4.rfind('/') {
                            let path5 = &path4[index+1..];
                            println!("{}", path5);
                        }
                        path4 = entry.path().into_os_string().into_string().unwrap();
                        path4.push_str("/");
                        path4.push_str("..");
                        if let Some(index) = path4.rfind('/') {
                            let path5 = &path4[index+1..];
                            println!("{}", path5);
                        }
                    }

                }else{
                    println!("{}", 176);
                    process::exit(176);
                }
            }
           


        }
    }else{
        println!("{}", 176);
        process::exit(176);
    }
    
}


fn chmod(args: &Vec<String>) -> i32{

    if args.len() < 3{
        println!("{}", 231);
        process::exit(231);
    }
    let mut numeric = 0;
    if let Ok(_) = args[2].parse::<u32>(){
        numeric = 1;
    }

    if numeric == 1{
        let num = &args[2];
        for arg in &args[3..]{
            let path = path::Path::new(&arg);
            let meta = fs::metadata(path).unwrap();
            let mut permissions = meta.permissions();let mode = u32::from_str_radix(num, 8).unwrap();
            
            permissions.set_mode(mode);
            if let Err(_) = fs::set_permissions(path, permissions){
                println!("{}", 231);
                process::exit(231);
            }
        }
    }else{  
        let codat = &args[2];
        let mut other = 0;
        let mut group=0;
        let mut user = 0;
        let mut all = 0;
        let mut plus = 0;
        let mut write = 0;
        let mut read = 0;
        let mut execute = 0;
        for c in codat.chars(){
            if c == 'o' && plus == 0 {
                other = 1;
            }
            if c =='g' && plus == 0{
                group = 1;
            }
            if c == 'u' && plus == 0{
                user = 1;
            }
            if c == 'a' && plus == 0{
                all = 1;
            }
            if c == '+'{
                plus = 1;
            }
            if c == '-'{
                plus = 2;
            }
            if c == 'r'{
                read = 1;
            }
            if c == 'w' {
                write = 1;
            }
            if c == 'x' {
                execute = 1;
            }
        }
        if other == 0 && group == 0 && user == 0 && all == 0{
            println!("Invalid command");
            process::exit(-1);
        }
        for arg in &args[3..]{
            let path =  path::Path::new(&arg);
            let meta = fs::metadata(path).unwrap();
            let mut permissions = meta.permissions();
            let mut mode = meta.permissions().mode();
            if plus == 1{
                if all == 1{
                    if read == 1{
                        mode |= 0o444;
                    }
                    if write == 1{
                        mode |= 0o222;
                    }
                    if execute == 1{
                        mode |= 0o111;
                    }
                }
                if other == 1{
                    if read == 1{
                        mode |= 0o004;
                    }
                    if write == 1{
                        mode |= 0o002;
                    }
                    if execute == 1{
                        mode |= 0o001;
                    }
                }
                if group == 1{
                    if read == 1{
                        mode |= 0o040;
                    }
                    if write == 1{
                        mode |= 0o020;
                    }
                    if execute == 1{
                        mode |= 0o010;
                    }
                }
        
                if user == 1{
                    if read == 1{
                        mode |= 0o400;
                    }
                    if write == 1{
                        mode |= 0o200;
                    }
                    if execute == 1{
                        mode |= 0o100;
                    }
                }
            }
            if plus == 2{
                if all == 1{
                    if read == 1{
                        mode -= 0o444;
                    }
                    if write == 1{
                        mode -= 0o222;
                    }
                    if execute == 1{
                        mode -= 0o111;
                    }
                }
                if other == 1{
                    if read == 1{
                        mode -= 0o004;
                    }
                    if write == 1{
                        mode -= 0o002;
                    }
                    if execute == 1{
                        mode -= 0o001;
                    }
                }
                if group == 1{
                    if read == 1{
                        mode -= 0o040;
                    }
                    if write == 1{
                        mode -= 0o020;
                    }
                    if execute == 1{
                        mode -= 0o010;
                    }
                }
        
                if user == 1{
                    if read == 1{
                        mode -= 0o400;
                    }
                    if write == 1{
                        mode -= 0o200;
                    }
                    if execute == 1{
                        mode -= 0o100;
                    }
                }
            }
            permissions.set_mode(mode);
            if let Err(_) = fs::set_permissions(path, permissions){
                println!("{}", 231);
                process::exit(231);
            }
        }
    }   

    0
}



fn main() {

    let mut ok = 0;
    let args: Vec<String> = env::args().collect();
        if args[1] == "pwd"{
            pwd();
            ok = 1;
        }
        if args[1] == "echo"{
            echo(& args);
            ok = 1;
        }
        if args[1] == "cat"{
            cat(& args);
            ok = 1;
            
        }
        if args[1] == "mkdir"{
            mkdir(& args);
            ok = 1;
        }
        if args[1] == "mv"{
            mv(& args);
            ok = 1;
        }
        if args[1] == "touch"{
            touch(& args);
            ok = 1;
        }
        if args[1] == "ln"{
           ln(& args);
           ok = 1;
        }
        if args[1] == "rmdir"{
            rmdir(& args);
            ok = 1;
        }
        if args[1] == "rm"{
           rm(& args);
            ok = 1;
        }
        if args[1] == "cp"{
            cp(& args);
            ok = 1;
        }
        if args[1] == "ls"{
            ls(&args);
            ok=1;
        }
        if args[1] == "chmod"{
            chmod(&args);
            ok = 1;
        }
        if ok == 0{
            println!("Invalid command");
            process::exit(-1);
        }
}