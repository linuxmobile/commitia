type JavaScriptResult = {
  functions: string[];
  constants: string[];
};

type NixResult = {
  constants: string[];
};

type JsonKeysResult = {
  keys: string[];
};


async function JAVASCRIPT(context: string): Promise<JavaScriptResult> {
  const functionRegex = /function\s+([a-zA-Z0-9_]+)\s*\(/g;
  const constRegex = /const\s+([a-zA-Z0-9_]+)\s*=/g;
  const objectConstRegex = /([a-zA-Z0-9_]+)\s*:\s*{/g;
  const destructuringConstRegex = /const\s*{([^}]+)}\s*=\s*([^;]+);/g;

  let match;
  const functionNames: string[] = [];
  const constNames: string[] = [];

  while ((match = functionRegex.exec(context)) !== null) {
    functionNames.push(match[1]);
  }

  while ((match = constRegex.exec(context)) !== null) {
    constNames.push(match[1]);
  }

  while ((match = objectConstRegex.exec(context)) !== null) {
    constNames.push(match[1]);
  }

  while ((match = destructuringConstRegex.exec(context)) !== null) {
    const destructuredVars = match[1].split(',').map(v => v.trim());
    constNames.push(...destructuredVars);
  }

  const result: JavaScriptResult = {
    functions: functionNames,
    constants: constNames
  };

  return result;
}

async function NIX(context: string): Promise<NixResult> {
  const keyRegex = /{([^}]+)}/;
  const match = keyRegex.exec(context);

  const constants = match ? match[1].split(',').map(key => key.trim()) : [];

  const result: NixResult = {
    constants
  };

  return result;
}

async function JSON_KEYS(context: string): Promise<JsonKeysResult> {
  const jsonObject = JSON.parse(context);
  const keys = Object.keys(jsonObject);

  const result: JsonKeysResult = {
    keys
  };

  return result;
}

const RULES = { JAVASCRIPT, NIX, JSON_KEYS };

export { RULES };