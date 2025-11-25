use clap::Parser;
use sqlx::{postgres::PgPoolOptions, Column, Row, TypeInfo, ValueRef};
use std::env;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Table name to generate seeder for
    #[arg(short, long)]
    table: String,

    /// Output file path (optional, default: migrations/seed_{table}.sql)
    #[arg(short, long)]
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env if exists (though in docker it might be env vars)
    let _ = dotenvy::dotenv();
    
    let args = Args::parse();
    let table_name = args.table;
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 1. Get Primary Key columns
    let pks: Vec<String> = sqlx::query(
        r#"
        SELECT a.attname
        FROM   pg_index i
        JOIN   pg_attribute a ON a.attrelid = i.indrelid
                             AND a.attnum = ANY(i.indkey)
        WHERE  i.indrelid = $1::regclass
        AND    i.indisprimary
        "#
    )
    .bind(&table_name)
    .fetch_all(&pool)
    .await?
    .iter()
    .map(|row| row.get::<String, _>("attname"))
    .collect();

    // 2. Get all data
    let query = format!("SELECT * FROM {}", table_name);
    let rows = sqlx::query(&query).fetch_all(&pool).await?;

    if rows.is_empty() {
        println!("No data found in table '{}'", table_name);
        return Ok(());
    }

    // 3. Generate SQL
    let mut sql = String::new();
    
    // Header
    sql.push_str(&format!("-- Seeder for table {}\n", table_name));
    
    // Columns
    let col_names: Vec<String> = rows[0].columns().iter().map(|c| c.name().to_string()).collect();
    let col_types: Vec<String> = rows[0].columns().iter().map(|c| c.type_info().name().to_string()).collect();
    let col_list = col_names.join(", ");

    for row in rows {
        let mut values = Vec::new();
        for (i, _) in col_names.iter().enumerate() {
            let val_ref = row.try_get_raw(i)?;
            
            if val_ref.is_null() {
                values.push("NULL".to_string());
            } else {
                // Simple type handling based on type name
                let type_name = col_types[i].as_str();
                
                let val_str = match type_name {
                    "BOOL" => {
                        match row.try_get::<bool, _>(i) {
                            Ok(v) => v.to_string(),
                            Err(_) => "NULL".to_string(),
                        }
                    },
                    "INT4" | "SERIAL" => {
                        match row.try_get::<i32, _>(i) {
                            Ok(v) => v.to_string(),
                            Err(_) => "NULL".to_string(),
                        }
                    },
                    "INT8" | "BIGSERIAL" => {
                        match row.try_get::<i64, _>(i) {
                            Ok(v) => v.to_string(),
                            Err(_) => "NULL".to_string(),
                        }
                    },
                    "FLOAT4" => {
                        match row.try_get::<f32, _>(i) {
                            Ok(v) => v.to_string(),
                            Err(_) => "NULL".to_string(),
                        }
                    },
                    "FLOAT8" => {
                        match row.try_get::<f64, _>(i) {
                            Ok(v) => v.to_string(),
                            Err(_) => "NULL".to_string(),
                        }
                    },
                    "VARCHAR" | "TEXT" | "CHAR" | "NAME" => {
                        match row.try_get::<String, _>(i) {
                            Ok(v) => format!("'{}'", v.replace("'", "''")),
                            Err(_) => "NULL".to_string(),
                        }
                    },
                    "TIMESTAMP" | "TIMESTAMPTZ" | "DATE" => {
                        // Try to get as chrono types first
                        use sqlx::types::chrono::{NaiveDateTime, DateTime, Utc, NaiveDate};
                        
                        if let Ok(v) = row.try_get::<DateTime<Utc>, _>(i) {
                            format!("'{}'", v.format("%Y-%m-%d %H:%M:%S%.f %:z"))
                        } else if let Ok(v) = row.try_get::<NaiveDateTime, _>(i) {
                            format!("'{}'", v.format("%Y-%m-%d %H:%M:%S%.f"))
                        } else if let Ok(v) = row.try_get::<NaiveDate, _>(i) {
                            format!("'{}'", v.format("%Y-%m-%d"))
                        } else {
                            "NULL".to_string()
                        }
                    },
                    _ => {
                        // Fallback: try String
                        match row.try_get::<String, _>(i) {
                            Ok(v) => format!("'{}'", v.replace("'", "''")),
                            Err(_) => "NULL".to_string(),
                        }
                    }
                };
                values.push(val_str);
            }
        }
        
        let val_list = values.join(", ");
        
        let mut insert_stmt = format!("INSERT INTO {} ({}) VALUES ({})", table_name, col_list, val_list);

        if !pks.is_empty() {
            let pk_list = pks.join(", ");
            let mut updates = Vec::new();
            for col in &col_names {
                if !pks.contains(col) {
                    updates.push(format!("{} = EXCLUDED.{}", col, col));
                }
            }
            
            if !updates.is_empty() {
                insert_stmt.push_str(&format!(" ON CONFLICT ({}) DO UPDATE SET {};", pk_list, updates.join(", ")));
            } else {
                insert_stmt.push_str(&format!(" ON CONFLICT ({}) DO NOTHING;", pk_list));
            }
        } else {
            insert_stmt.push_str(";");
        }
        
        sql.push_str(&insert_stmt);
        sql.push('\n');
    }

    // 4. Output
    let output_path = args.output.unwrap_or_else(|| format!("migrations/seed_{}.sql", table_name));
    let mut file = File::create(&output_path)?;
    file.write_all(sql.as_bytes())?;
    
    println!("Seeder generated at: {}", output_path);

    Ok(())
}
