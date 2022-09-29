import { useAppDispatch, useTypedSeletor } from '../hooks/store'
import {
  decrement,
  increment,
  incrementByAmount,
} from '../features/counter/counterSlice'

export default () => {
  const count = useTypedSeletor((state) => state.counter.value)
  const dispatch = useAppDispatch()

  return (
    <div>
      <div>counter: {count}</div>
      <div className='flex gap-4'>
        <button
          className='px-4 py-2 bg-slate-500 text-white'
          onClick={() => {
            dispatch(increment())
          }}
        >
          add
        </button>
        <button
          className='px-4 py-2 bg-slate-500 text-white'
          onClick={() => {
            dispatch(decrement())
          }}
        >
          add
        </button>
        <button
          className='px-4 py-2 bg-slate-500 text-white'
          onClick={() => {
            dispatch(incrementByAmount(3))
          }}
        >
          add amount
        </button>
      </div>
    </div>
  )
}
