from flask import Flask, request
from datetime import datetime

app = Flask(__name__)
print("INFO: Starting Flask Server at http://localhost:5000")


@app.route('/health')
def health():
    return f"Alive at {datetime.now()}"


@app.route('/add')
def add():
    num1: int = int(request.args.get('num1'))
    num2: int = int(request.args.get('num2'))
    return f"{num1} + {num2} = {num1 + num2}"


@app.route('/subtract')
def subtract():
    num1: int = int(request.args.get('num1'))
    num2: int = int(request.args.get('num2'))
    return f"{num1} - {num2} = {num1 - num2}"


@app.route('/multiply')
def multiply():
    num1: int = int(request.args.get('num1'))
    num2: int = int(request.args.get('num2'))
    return f"{num1} * {num2} = {num1 * num2}"


@app.route('/divide')
def divide():
    num1: int = int(request.args.get('num1'))
    num2: int = int(request.args.get('num2'))
    return f"{num1} / {num2} = {num1 / num2}"
