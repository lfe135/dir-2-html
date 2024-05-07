use std::{ fs::{self, create_dir_all, File}, io::{self, Read, Write}, path::{Path, PathBuf}};

use log::trace;
use simple_logger::SimpleLogger;

fn main() ->io::Result<()> {
    SimpleLogger::new().init().unwrap();
    let root = Path::new("C:\\Users\\lfe135\\Downloads\\edge\\kubernetes-master\\apiserver");
    let html = Path::new(".\\html");
    Dir2Html{root, html}.create()
}

struct  Dir2Html<'a>{
    root: &'a Path,
    html: &'a Path,
}

impl<'a>  Dir2Html<'a> {
    fn create_code_viewer_html_file(&self, path: &PathBuf) -> io::Result<()>{
        trace!("canonicalize {}", path.display());
        let path = fs::canonicalize(path)?;
        trace!("open {}", path.display());
        let mut reader = File::open(&path)?;
        let mut content = String::new();
        trace!("read {}", path.display());
        reader.read_to_string(&mut content)?;
        content = content.replace("<", "&lt;");
        content = content.replace(">", "&gt;");
        trace!("new extension");
        let new_ext = match path.extension() {
            Some(ext) => {
                let ext = ext.to_str().ok_or(io::ErrorKind::InvalidInput)?;
                format!("{ext}.html")
            },
            None => {
                "html".into()
            }
        };
        trace!("create code view html  file dir {}", path.display());
        let path = self.create_code_viewer_html_file_dir(&path)?;
        let file_path = path.with_extension(new_ext);
        trace!("create file {}", file_path.display());
        let mut writer = File::create(&file_path)?;
        trace!("write content");
        write!(&mut writer, "<!DOCTYPE html><html><body><pre style=\"white-space: pre-wrap\">\n{content}</pre></body></html>")
    }
    fn visit_dirs(&self, dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let _ = self.visit_dirs(&path);
                } else {
                    trace!("create code viewer html file {}", path.display());
                    let _ = self.create_code_viewer_html_file(&path);
                }
            }
            trace!("create dir index html file for {}", dir.display());
            self.create_dir_index_html_file(dir)?;
        }
        Ok(())
    }
    fn create_dir_index_html_file(&self, dir: &Path) -> io::Result<()> {
        let mut body = String::new();
        trace!("read dir {}", dir.display());
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Ok(path_str) = path.strip_prefix(self.root) {
                let html_name = self.html.file_name().ok_or(io::ErrorKind::Other)?.to_str().unwrap_or_default();
                body += &format!("<div><a href=\"\\{}\\{}.html\">{}</a></div>", html_name, path_str.display(),  path_str.display());
            }
        }
        trace!("create index html file dir {}", dir.display());
        let file_dir = self.create_index_html_file_dir(dir)?;
        let file_path = file_dir.with_extension("html");
        trace!("create file {}", file_path.display());
        let mut file  =  File::create(&file_path)?;
        trace!("write file {}", file_path.display());
        write!(&mut file, "<!DOCTYPE html><html><body>{body}</body></html>")
    }
    fn create_index_html_file_dir(&self,  dir: &Path)  -> io::Result<PathBuf> {
        let absolute_dir_path = fs::canonicalize(dir)?;
    
        let absolute_root_path = fs::canonicalize(self.root)?;
    
        create_dir_all(self.html)?;
        let absolute_html_path = fs::canonicalize(self.html)?;
    
        let relative_dir_path = absolute_dir_path.strip_prefix(&absolute_root_path).unwrap();
        let absolute_html_dir_path = absolute_html_path.join(relative_dir_path);
        
        create_dir_all(&absolute_html_dir_path)?;
    
        Ok(absolute_html_dir_path)
    }
    fn create_code_viewer_html_file_dir(&self,  file: &Path)  -> io::Result<PathBuf> {
        let absolute_file_path = fs::canonicalize(file)?;
    
        let absolute_root_path = fs::canonicalize(self.root)?;
    
        create_dir_all(self.html)?;
        let absolute_html_path = fs::canonicalize(self.html)?;
    
        let relative_file_path = absolute_file_path.strip_prefix(&absolute_root_path).unwrap();
        let absolute_html_file_path = absolute_html_path.join(relative_file_path);
        
        let absolute_html_file_parent_path = absolute_html_file_path.parent().ok_or(io::ErrorKind::NotFound)?;
        create_dir_all(&absolute_html_file_parent_path)?;
    
        Ok(absolute_html_file_path)
    }
    fn create(&'a mut self) -> io::Result<()>{
        let  root = self.root.canonicalize()?;
        self.visit_dirs(&root)
    }
}