async function JAVASCRIPT(context) {
  const functionRegex = /function\s+([a-zA-Z0-9_]+)\s*\(/g;
  const constRegex = /const\s+([a-zA-Z0-9_]+)\s*=/g;

  let match;
  const functionNames = [];
  const constNames = [];

  while ((match = functionRegex.exec(context)) !== null) {
    functionNames.push(match[1]);
  }

  while ((match = constRegex.exec(context)) !== null) {
    constNames.push(match[1]);
  }

  const result = {
    functions: functionNames,
    constants: constNames
  };

  return result;
}

async function TYPESCRIPT(context) {
  const functionRegex = /function\s+([a-zA-Z0-9_]+)\s*\(/g;
  const constRegex = /const\s+([a-zA-Z0-9_]+)\s*=/g;

  let match;
  const functionNames = [];
  const constNames = [];

  while ((match = functionRegex.exec(context)) !== null) {
    functionNames.push(match[1]);
  }

  while ((match = constRegex.exec(context)) !== null) {
    constNames.push(match[1]);
  }

  const result = {
    functions: functionNames,
    constants: constNames
  };

  return result;
}

async function NIX(context) {
  const keyRegex = /{([^}]+)}/;
  const match = keyRegex.exec(context);

  const constants = match ? match[1].split(',').map(key => key.trim()) : [];

  const result = {
    constants
  };

  return result;
}

async function JSON_KEYS(context) {
  const jsonObject = JSON.parse(context);
  const keys = Object.keys(jsonObject);

  const result = {
    keys
  };

  return result;
}

const RULES = { JAVASCRIPT, TYPESCRIPT, NIX, JSON_KEYS };

export { RULES };