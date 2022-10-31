import { useCallback, useState } from 'react'
import { calculate_pi as calculatePi } from '../../calculator/pkg'
import { wasmFn } from '@/utility/wasm'

export default function Pi() {
  const [result, setResult] = useState(0.0)

  const handleClick = useCallback(async () => {
    setResult(await wasmFn(calculatePi))
  }, [])
  return (
    <main className="min-h-screen w-screen flex justify-center items-center flex-col bg-amber-900">
      <button onClick={handleClick}>click</button>
      <div className="text-amber-50">Hello {result}</div>
    </main>
  )
}
