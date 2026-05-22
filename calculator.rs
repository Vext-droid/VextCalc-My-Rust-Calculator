use std::io::{self, Write};
use std::fs;
use std::fs::OpenOptions;

fn sum(n1: f64, n2: f64) -> f64 {
n1 + n2
}
fn subtract(n1: f64, n2: f64) -> f64 {
n1 - n2
}
fn multiply(n1: f64, n2: f64) -> f64 {
n1 * n2
}
fn divide(n1: f64, n2: f64) -> f64 {
if n2.abs() < 1e-10 {
println!("Impossible to divide by 0");
0.0
}
else {
n1 / n2
}
}
fn read_number() -> f64 {
loop {
print!("Type a number ");
io::stdout().flush().unwrap();
let mut n1 = String::new();
io::stdin().read_line(&mut n1).unwrap();
match n1.trim().parse::<f64>() {
Ok(n) => return n,
Err(_) => print!("Invalid Number try again"),
}
}
}
fn get_numbers() -> (f64, f64) {
let n1 = read_number();
let n2 = read_number();
(n1,n2)
}
fn calc() {
println!("\n1- Sum");
println!("2- Subtract");
println!("3- multiply");
println!("4- divide");
println!("5- History");
print!("Choose ");
io::stdout().flush().unwrap();
let mut option = String::new();
io::stdin().read_line(&mut option).unwrap();
let option = option.trim();

let (n1,n2) = get_numbers();
match option.trim() {
"1" => {
println!("Result = {}\n", sum(n1,n2));
let operation = format!("{} + {} = {}", n1,n2,
sum(n1,n2));
let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("calc.txt")
.unwrap();

writeln!(file, "{}", operation).unwrap();
}
"2" => {
println!("Result = {}\n", subtract(n1,n2));
let operation = format!("{} - {} = {}", n1,n2,
subtract(n1,n2));
let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("calc.txt")
.unwrap();

writeln!(file, "{}", operation).unwrap();
}
"3" => {
println!("Result = {}\n", multiply(n1,n2));
let operation = format!("{} * {} = {}", n1,n2,
multiply(n1,n2));
let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("calc.txt")
.unwrap();

writeln!(file, "{}", operation).unwrap();
}
"4" => {
if n2.abs() < 1e-10 {
println!("Impossible to divide by 0\n");
}
else {
println!("Result = {}\n", divide(n1,n2));
let operation = format!("{} / {} = {}",n1,n2,
divide(n1,n2));
let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("calc.txt")
.unwrap();

writeln!(file, "{}", operation).unwrap();
}
}
"5" => {
let content =
fs::read_to_string("calc.txt").unwrap();

println!("{}", content);
}
_=> {
println!("Invalid Option");
}
}
}
fn read_signalabc() -> String {
loop {
print!("\nPositive or Negative? [+/-] ");
io::stdout().flush().unwrap();
let mut signal = String::new();
io::stdin().read_line(&mut signal).unwrap();
let signal = signal.trim().to_string();

if signal == "+" || signal == "-" {
return signal;
}
println!("Invalid signal");
}
}
fn read_value(msg: &str) -> f64 {
loop {
print!("{}", msg);
io::stdout().flush().unwrap();
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
match input.trim().parse::<f64>() {
Ok(n) => return n,
Err(_) => print!("Invalid number try again"),
}
}
}
fn getabc() -> (f64,f64,f64,String,String,
String) {
let mut a = read_value("Enter the value of A ");
let signal_a = read_signalabc();
if signal_a == "-" {
a = -a;
}
let mut b = read_value("Enter the value of B ");
let signal_b = read_signalabc();
if signal_b == "-" {
b = -b;
}
let mut c = read_value("Enter the value of C ");
let signal_c = read_signalabc();
if signal_c == "-" {
c = -c;
}
(a,b,c,signal_a,signal_b,signal_c)
}
fn bhaskara() {
let (a,b,c,signal_a,signal_b,signal_c) =
getabc();
let delta = b * b - 4.0 * a * c;
let epsilon = 1e-10;
println!("\x1b[1m\nThe Equation");
println!("{}{}X² {}{} {}{} = 0\x1b[0m\n", 
signal_a,a,signal_b,b,signal_c,c);
println!("\nFind The Value of ABC");
println!("A = {}{}, B = {}{}, C = {}{}\n",
signal_a,a,signal_b,b,signal_c,c);
println!("Find The Value of Delta");
if c.is_sign_negative() {
println!("∆=(B)²-4AC");
println!("∆=({})²={}", b, b * b);
println!("4 * {}{} * -({}) = {}\n",
signal_a,a,c, 4.0 * a * c);
println!("∆= {} - {}",
b * b, 4.0 * a * c);
println!("∆={}\n", delta);

if delta < -epsilon {
let equation = format!(r#"
{}X² {}{}X {}{} = 0

Result: There's no root

"#, a,signal_b,b,signal_c,c);
println!("\nThere's no Root\n");
let mut files = OpenOptions::new()
.append(true)
.create(true)
.open("bhaskara.txt")
.unwrap();

writeln!(files, "{}", equation).unwrap();
}
else if delta.abs() < -epsilon {
let x = -b / (2.0 * a);
println!("\nThere's one root");
println!("X¹ = {}\n", x);
let equation = format!(r#"

{}X² {}{}x {}{} = 0

Result: {}
"#,a,signal_b,b,signal_c,c,x);
let mut files = OpenOptions::new()
.append(true)
.create(true)
.open("bhaskara.txt")
.unwrap();

writeln!(files, "{}", equation).unwrap();
}
else {
let sqrt_delta = delta.sqrt();
let x1 = (-b + sqrt_delta) / (2.0 * a);
let x2 = (-b - sqrt_delta) / (2.0 * a);
println!("\nX¹ = {}", x1);
println!("X² = {}\n", x2);
let equation = format!(r#"

{}X² {}{}x {}{} = 0

Result: {{{},{}}}
"#,a,signal_b,b, signal_c,c,x1,x2);
let mut files = OpenOptions::new()
.append(true)
.create(true)
.open("bhaskara.txt")
.unwrap();

writeln!(files, "{}", equation).unwrap();
}
}
else {
println!("∆=(B)²-4AC");
println!("∆=({})²={}", b, b * b);
println!("4 * {}{} * {} = {}\n",
signal_a,a,c, 4.0 * a * c);
println!("∆= {} - {}",
b * b, 4.0 * a * c);
println!("∆={}\n", delta);

if delta < -epsilon {
let equation = format!(r#"
{}X² {}{}X {}{} = 0

Result: There's no root

"#, a,signal_b,b,signal_c,c);
println!("\nThere's no Root\n");
let mut files = OpenOptions::new()
.append(true)
.create(true)
.open("bhaskara.txt")
.unwrap();

writeln!(files, "{}", equation).unwrap();
}
else if delta.abs() < -epsilon {
let x = -b / (2.0 * a);
println!("\nThere's one root");
println!("X¹ = {}\n", x);
let equation = format!(r#"

{}X² {}{}x {}{} = 0

Result: {}
"#,a,signal_b,b,signal_c,c,x);
let mut files = OpenOptions::new()
.append(true)
.create(true)
.open("bhaskara.txt")
.unwrap();

writeln!(files, "{}", equation).unwrap();
}
else {
let sqrt_delta = delta.sqrt();
let x1 = (-b + sqrt_delta) / (2.0 * a);
let x2 = (-b - sqrt_delta) / (2.0 * a);
println!("\nX¹ = {}", x1);
println!("X² = {}\n", x2);
let equation = format!(r#"

{}X² {}{}x {}{} = 0

Result: {{{},{}}}
"#, a,signal_b,b, signal_c,c,x1,x2);
let mut files = OpenOptions::new()
.append(true)
.create(true)
.open("bhaskara.txt")
.unwrap();

writeln!(files, "{}", equation).unwrap();
}
}
}
fn bhaskaras() {
println!("\n1- Resolve Equations With Bhaskara's formula");
println!("2- History");
print!("Choose ");
io::stdout().flush().unwrap();
let mut option = String::new();
io::stdin().read_line(&mut option).unwrap();
let option = option.trim();

match option.trim() {
"1" => { 
bhaskara();
}
"2" => {
let content =
fs::read_to_string("bhaskara.txt").unwrap();

println!("{}", content);
}
_=> {
println!("Invalid Option");
}
}
}
fn read_numbers(msg: &str) -> f64 {
loop {
print!("{}", msg);
io::stdout().flush().unwrap();
let mut numbers = String::new();
io::stdin().read_line(&mut numbers).unwrap();
match numbers.trim().parse::<f64>() {
Ok(n) => return n,
Err(_) => print!("Invalid number try again"),
}
}
}
fn get_numbers4() -> (
f64,f64,f64,f64,
String,String,String,String) {

let mut x = read_numbers("Enter a Value to X ");
let signal_x = read_signalabc();
if signal_x == "-" {
x = -x;
}
let mut x2 = read_numbers("Enter another Value to X ");
let signal_x2 = read_signalabc();
if signal_x2 == "-" {
x2 = -x2;
}
let mut n1 = read_numbers("Type a number ");
let signal_n1 = read_signalabc();
if signal_n1 == "-" {
n1 = -n1;
}
let mut n2 = read_numbers("Type another number");
let signal_n2 = read_signalabc();
if signal_n2 == "-" {
n2 = -n2;
}
(x,x2,n1,n2,signal_x,signal_x2,signal_n1,signal_n2)
}
fn equation() {
let (
x,x2,n1,n2,signal_x,signal_x2,signal_n1,signal_n2
) = get_numbers4();

println!("\nThe Equation");
println!("{}{}x {}{} = {}{}x {}{}\n",signal_x,x,
signal_n1,n1,signal_x2,x2,signal_n2,n2);
println!("\x1b[1m\nSeparate Number from letter\x1b[0m");

if x2.is_sign_positive() && n1.is_sign_positive() {
println!("{}x - {}x = {} - {}\n",x,x2,n1,n2);
println!("{}x - {}x = {}",x,x2,x - x2);
println!("{} - {} = {}\n",n1,n2,n1 - n2);
println!("{}x = {}", x - x2, n1 - n2);
println!("X = {}\n",(n1 - n2) / (x - x2));
let result = (n1 - n2) / (x - x2);
let equation = format!(r#"
{}x {}{} = {}x {}{}

X = {}
"#,x,signal_n1,n1,x2,signal_n2,n2,result);
let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("Equations.txt")
.unwrap();

writeln!(file, "{}", equation).unwrap();
}
else if x2.is_sign_negative() && n1.is_sign_negative() {
println!("{}x + {}x = {} + {}\n",x,x2,n1,n2);
println!("{}x + {}x = {}",x,x2,x + x2);
println!("{} + {} = {}\n",n1,n2,n1 + n2);
println!("{}x = {}", x + x2, n1 + n2);
println!("X = {}\n",(n1 + n2) / (x + x2));
let result = (n1 + n2) / (x + x2);
let equation = format!(r#"
{}x {}{} = {}x {}{}

X = {}
"#,x,signal_n1,n1,x2,signal_n2,n2,result);
let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("Equations.txt")
.unwrap();

writeln!(file, "{}", equation).unwrap();
}
else if x2.is_sign_negative() && n1.is_sign_positive() {
println!("{}x + {}x = {} - {}\n",x,x2,n1,n2);
println!("{}x + {}x = {}",x,x2,x + x2);
println!("{} - {} = {}\n",n1,n2,n1 - n2);
println!("{}x = {}", x + x2, n1 - n2);
println!("X = {}\n", (n1 - n2) / (x + x2));
let result = (n1 - n2) / (x + x2);
let equation = format!(r#"
{}x {}{} = {}x {}{}

X = {}
"#,x,signal_n1,n1,x2,signal_n2,n2,result);
let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("Equations.txt")
.unwrap();

writeln!(file, "{}", equation).unwrap();
}
else if x2.is_sign_positive() && n1.is_sign_negative() {
println!("{}x - {}x = {} + {}\n",x,x2,n1,n2);
println!("{}x - {}x = {}",x,x2,x - x2);
println!("{} + {} = {}\n",n1,n2,n1 + n2);
println!("{}x = {}", x - x2, n1 + n2);
println!("X = {}\n", (n1 + n2) / (x - x2));
let result = (n1 + n2) / (x - x2);
let equation = format!(r#"
{}x {}{} = {}x {}{}

X = {}
"#,x,signal_n1,n1,x2,signal_n2,n2,result);
let mut file = OpenOptions::new()
.append(true)
.create(true)
.open("Equations.txt")
.unwrap();

writeln!(file, "{}", equation).unwrap();
}
}
fn equations() {
println!("\n1- Resolve First Degree-Equations");
println!("2- Histrory");
print!("\nChoose ");
io::stdout().flush().unwrap();
let mut options = String::new();
io::stdin().read_line(&mut options).unwrap();
let options = options.trim();

match options.trim() {
"1" => {
equation();
}
"2" => {
let content =
fs::read_to_string("Equations.txt").unwrap();

println!("{}", content);
}
_=> {
println!("Invalid Option");
}
}
}
fn calculator() -> bool {
println!("\n1- Solve Simple Operations");
println!("2- Solve First Degree-Equations");
println!("3- Solve quadratic equations using Bhaskara's formula");
println!("4- Exit");
println!("5- list all accounts created");
print!("\nChoose ");
io::stdout().flush().unwrap();
let mut choice = String::new();
io::stdin().read_line(&mut choice).unwrap();
let choice = choice.trim();

match choice.trim() {
"1" => {
calc();
}
"2" => {
equations();
}
"3" => {
bhaskaras();
}
"4" => {
return false;
}
"5" => {
let calcs =
fs::read_to_string("calc.txt").unwrap();

let bhaskara =
fs::read_to_string("bhaskara.txt").unwrap();

let equations =
fs::read_to_string("Equations.txt").unwrap();

println!("{}", calcs);
println!("{}", equations);
println!("{}", bhaskara);
}
_=> {
println!("Invalid Option");
}
}
true
}
fn main() {
println!("\nWelcome To VextCalc!\n");
loop {
if !calculator() {
break;
}
}
}
