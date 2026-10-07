// Re-export types and commands from the generated bindings.
// This module exists so the rest of the app imports from '#lib/ipc/index.js'
// rather than directly from the generated file.

export type { AppError, Cell, GameSnapshot, PuzzleSpec } from './bindings';
export { commands } from './bindings';
