import { useGetPostQuery } from '../app/services/posts'
import { useAddTodoMutation, useGetTodosQuery } from '../app/services/todos'

export default () => {
  const {
    data: todos,
    isLoading,
    isSuccess,
    isError,
    error,
  } = useGetTodosQuery()

  const { data: post, isSuccess: ok } = useGetPostQuery(1)

  const [addTodo] = useAddTodoMutation()
  let content

  if (isLoading) content = <div>loading</div>
  if (isSuccess)
    content = (
      <div className=''>
        <div>
          <button
            className='p-4 bg-slate-500'
            onClick={() => {
              addTodo({ title: 'title' })
            }}
          >
            ADD
          </button>
        </div>

        {ok && (
          <div className='border border-black p-4 my-2 bg-slate-300'>
            <p>postid: {post.id}</p>
            <p>title: {post.title}</p>
          </div>
        )}

        {todos.map((todo) => (
          <div
            key={todo.id}
            className='border border-black p-4 my-2 bg-slate-300'
          >
            <p>user: {todo.userId}</p>
            <p>title: {todo.title}</p>
            <p>completed: {todo.completed!.toString()}</p>
          </div>
        ))}
      </div>
    )

  if (isError) content = <div>{error.toString()}</div>

  return <div>{content}</div>
}
