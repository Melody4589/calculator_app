let expression = '';

function clearDisplay() {
    expression = '';
    document.getElementById('expressionDisplay').textContent = '';
    document.getElementById('resultDisplay').textContent = '0';
}

function appendCharacter(char) {
    expression += char;
    document.getElementById('expressionDisplay').textContent = expression;
}

async function calculate() {
    const response = await fetch('/calculate', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/x-www-form-urlencoded'
        },
        body: new URLSearchParams({ expression })
    });
    const result = await response.json();
    document.getElementById('resultDisplay').textContent = result.result;
}
