import wasmModule from '@/../calculator/pkg'

export async function wasmFn<T>(fn: () => T) {
  return wasmModule().then(() => fn())
}
