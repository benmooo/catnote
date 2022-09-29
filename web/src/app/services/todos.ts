import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'

interface Todo {
  id?: number
  userId?: number
  title: string
  completed?: boolean
}

type TodosResponse = Todo[]

export const jsonApi = createApi({
  reducerPath: 'jsonApi',
  baseQuery: fetchBaseQuery({
    baseUrl: 'https://jsonplaceholder.typicode.com',
  }),
  tagTypes: ['Todos'],
  endpoints: (builder) => ({
    getTodos: builder.query<TodosResponse, void>({
      query: () => '/todos',
      transformResponse: (res: TodosResponse) =>
        res.sort((a, b) => a.id! - b.id!),
      providesTags: ['Todos'],
    }),

    addTodo: builder.mutation({
      query: (todo: Todo) => ({
        url: '/todos',
        method: 'POST',
        body: todo,
      }),
      invalidatesTags: ['Todos'],
    }),

    deleteTodo: builder.mutation({
      query: (id: number) => ({
        url: `/todos/${id}`,
        method: 'Delete',
      }),
      invalidatesTags: ['Todos'],
    }),
  }),
})

export const { useGetTodosQuery, useAddTodoMutation } = jsonApi
