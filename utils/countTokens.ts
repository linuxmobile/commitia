import { encode } from 'gpt-tokenizer/model/gpt-4o';

function tokenCount(content: string): number {
  const tokens = encode(content);
  return tokens.length;
}

export { tokenCount };