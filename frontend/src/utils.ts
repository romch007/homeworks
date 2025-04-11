export function randomChoice<T>(sequence: T[]): T {
  return sequence[Math.floor(Math.random() * sequence.length)];
}
