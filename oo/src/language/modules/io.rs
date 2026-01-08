use crate::language::analyzing::analyzer::{Analyzer, FunctionSignature};
use crate::language::analyzing::types::VariableType;
use crate::language::running::interpreter::{Interpreter, RuntimeFunction};
use crate::language::running::value::Value;



impl Analyzer {
    pub fn register_io_functions(&mut self) {
        
        self.functions.insert(
            vec!["IO".to_string(), "readFile".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING],              
                return_type: VariableType::STRING, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "printFile".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING, VariableType::BOOLEAN],              
                return_type: VariableType::NULL, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "writeFile".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING, VariableType::STRING],              
                return_type: VariableType::NULL, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "appendFile".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING, VariableType::STRING],              
                return_type: VariableType::NULL, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "size".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING],              
                return_type: VariableType::INT, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "exists".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING],              
                return_type: VariableType::BOOLEAN, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "isFile".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING],              
                return_type: VariableType::BOOLEAN, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "isDirectory".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING],              
                return_type: VariableType::BOOLEAN, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "deleteFile".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING],              
                return_type: VariableType::NULL, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "deleteDirectory".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING],              
                return_type: VariableType::NULL, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "move".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING, VariableType::STRING],              
                return_type: VariableType::NULL, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "copy".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING, VariableType::STRING],              
                return_type: VariableType::NULL, 
                called: true
            },
        );

        self.functions.insert(
            vec!["IO".to_string(), "lines".to_string()],
            FunctionSignature {
                parameters: vec![VariableType::STRING],              
                return_type: VariableType::INT, 
                called: true
            },
        );

    }
}

impl Interpreter {

    pub fn register_io_functions(&mut self) {

        self.functions.insert(
            vec!["IO".to_string(), "readFile".to_string()],
            RuntimeFunction::Native(Self::read_file),
        );

        self.functions.insert(
            vec!["IO".to_string(), "printFile".to_string()],
            RuntimeFunction::Native(Self::print_file),
        );

        self.functions.insert(
            vec!["IO".to_string(), "writeFile".to_string()],
            RuntimeFunction::Native(Self::write_to_file),
        );

        self.functions.insert(
            vec!["IO".to_string(), "appendFile".to_string()],
            RuntimeFunction::Native(Self::append_to_file),
        );

        self.functions.insert(
            vec!["IO".to_string(), "size".to_string()],
            RuntimeFunction::Native(Self::size),
        );

        self.functions.insert(
            vec!["IO".to_string(), "exists".to_string()],
            RuntimeFunction::Native(Self::exists),
        );

        self.functions.insert(
            vec!["IO".to_string(), "isFile".to_string()],
            RuntimeFunction::Native(Self::is_file),
        );

        self.functions.insert(
            vec!["IO".to_string(), "isDirectory".to_string()],
            RuntimeFunction::Native(Self::is_directory),
        );

        self.functions.insert(
            vec!["IO".to_string(), "deleteDirectory".to_string()],
            RuntimeFunction::Native(Self::delete_directory),
        );

        self.functions.insert(
            vec!["IO".to_string(), "deleteFile".to_string()],
            RuntimeFunction::Native(Self::delete_file),
        );

        self.functions.insert(
            vec!["IO".to_string(), "move".to_string()],
            RuntimeFunction::Native(Self::move_file),
        );

        self.functions.insert(
            vec!["IO".to_string(), "copy".to_string()],
            RuntimeFunction::Native(Self::copy),
        );

        self.functions.insert(
            vec!["IO".to_string(), "lines".to_string()],
            RuntimeFunction::Native(Self::lines),
        );
        
    }
}

impl Interpreter {
    
    fn read_file(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 1 {
            return Err("IO.readFile expects exactly one argument".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.readFile expects a string".to_string()),
        };

        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("IO.readFile failed: {}", e))?;

        Ok(Value::STRING(contents))
    }

    fn print_file(args: Vec<Value>) -> Result<Value, String> {

        if args.len() != 2 {
            return Err("IO.printFile expects exactly two arguments".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.printFile expects a string as first argument".to_string()),
        };

        let numbered = match &args[1] {
            Value::BOOLEAN(b) => *b,
            _ => return Err("IO.printFile expects a boolean as second argument".to_string()),
        };

        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("IO.printFile could not read file: {}", e))?;

        if numbered {
            let total_lines = contents.lines().count();
            let width = total_lines.to_string().len();

            let mut counter = 0;
            for line in contents.lines() {
                counter += 1;
                let num_str = format!("{num:>width$}", num = counter, width = width);
                println!("{} {} {line}", num_str, "|", line = line);
            }   // TODO Colored Text
        } else {
            println!("{}", contents);
        }

        Ok(Value::NULL)
    }

    fn write_to_file(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("IO.writeFile expects exactly two arguments".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.writeFile expects a string path as the first argument".to_string()),
        };

        let contents = match &args[1] {
            Value::STRING(s) => s,
            _ => return Err("IO.writeFile expects a string as the second argument".to_string()),
        };

        std::fs::write(path, contents)
            .map_err(|e| format!("IO.writeFile failed: {}", e))?;

        Ok(Value::NULL)
    }

    fn append_to_file(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("IO.appendFile expects exactly two arguments".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.appendFile expects a string path".to_string()),
        };

        let contents = match &args[1] {
            Value::STRING(s) => s,
            _ => return Err("IO.appendFile expects a string".to_string()),
        };

        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("IO.appendFile failed: {}", e))?;

        file.write_all(contents.as_bytes())
            .map_err(|e| format!("IO.appendFile failed: {}", e))?;

        Ok(Value::NULL)
    }

    fn size(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("IO.size expects exactly one argument".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.size expects a string path argument".to_string()),
        };

        use std::fs;
        let data = fs::metadata(path)
            .map_err(|e| format!("IO.size failed: {}", e))?;


        Ok(Value::INT(data.len() as i32))
    }   // TODO Directory Size

    fn exists(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("IO.exists expects exactly one argument".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.exists expects a string".to_string()),
        };

        Ok(Value::BOOLEAN(std::path::Path::new(path).exists()))
    }

    fn is_file(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("IO.isFile expects exactly one argument".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.isFile expects a string".to_string()),
        };

        match std::fs::metadata(path) {
            Ok(meta) => Ok(Value::BOOLEAN(meta.is_file())),
            Err(_) => Ok(Value::BOOLEAN(false)),
        }
    }

    fn is_directory(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("IO.isDirectory expects exactly one argument".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.isDirectory expects a string".to_string()),
        };

        match std::fs::metadata(path) {
            Ok(meta) => Ok(Value::BOOLEAN(meta.is_dir())),
            Err(_) => Ok(Value::BOOLEAN(false)),
        }
    }

    fn delete_file(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("IO.deleteFile expects exactly one argument".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.deleteFile expects a string".to_string()),
        };

        std::fs::remove_file(path)
            .map_err(|e| format!("IO.deleteFile failed: {}", e))?;

        Ok(Value::NULL)
    }

    fn delete_directory(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("IO.deleteDirectory expects exactly one argument".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.deleteDirectory expects a string".to_string()),
        };

        std::fs::remove_dir(path)
            .map_err(|e| format!("IO.deleteDirectory failed: {}", e))?;

        Ok(Value::NULL)
    }

    fn move_file(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("IO.move expects exactly two arguments".to_string());
        }

        let src = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.move expects a string source path".to_string()),
        };

        let dst = match &args[1] {
            Value::STRING(s) => s,
            _ => return Err("IO.move expects a string destination path".to_string()),
        };

        std::fs::rename(src, dst)
            .map_err(|e| format!("IO.move failed: {}", e))?;

        Ok(Value::NULL)
    }

    fn copy(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("IO.copy expects exactly two arguments".to_string());
        }

        let src = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.copy expects a string source path".to_string()),
        };

        let dst = match &args[1] {
            Value::STRING(s) => s,
            _ => return Err("IO.copy expects a string destination path".to_string()),
        };

        let meta = std::fs::metadata(src)
            .map_err(|e| format!("IO.copy failed: {}", e))?;

        if !meta.is_file() {
            return Err("IO.copy only supports files".to_string());
        }

        std::fs::copy(src, dst)
            .map_err(|e| format!("IO.copy failed: {}", e))?;

        Ok(Value::NULL)
    }


    fn lines(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("IO.lines expects exactly one argument".to_string());
        }

        let path = match &args[0] {
            Value::STRING(s) => s,
            _ => return Err("IO.lines expects a string".to_string()),
        };

        let contents = std::fs::read_to_string(path)
            .map_err(|e| format!("IO.lines failed: {}", e))?;

        Ok(Value::INT(contents.lines().count() as i32))
    }

}