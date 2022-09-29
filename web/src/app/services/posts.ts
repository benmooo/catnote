import { createApi } from '@reduxjs/toolkit/query/react'
import { graphqlRequestBaseQuery } from '@rtk-query/graphql-request-base-query'
import { gql } from 'graphql-request'

export const postStatus = ['draft', 'published', 'pending review']
export interface Post {
  id: string
  title: string
  author: string
  content: string
  status: typeof postStatus[number]
  createdAt: string
  updatedAt: string
}

export interface Pagination {
  page: number
  perPage: number
  total: number
  totalPages: number
}

export interface GetPostsResponse extends Pagination {
  data: {
    posts: Post[]
  }
}

export interface PostResponse {
  data: {
    post: Post
  }
}

export const gqlApi = createApi({
  reducerPath: 'gqlApi',

  baseQuery: graphqlRequestBaseQuery({
    url: 'https://graphqlzero.almansi.me/api',
  }),
  endpoints: (builder) => ({
    // getPosts: builder.query<
    //   GetPostsResponse,
    //   { page?: number; perPage?: number }
    // >({
    //   query: ({ page, perPage }) => ({
    //     document: gql`
    //       query GetPosts($page: Int = 1, $perPage: Int = 10) {
    //         id
    //         title
    //       }
    //     }
    //   `,
    //     variables: { page, perPage },
    //   }),
    // }),

    getPost: builder.query<Post, number>({
      query: (id) => ({
        document: gql`
        query  {
          post(id: ${id}) {
            id
            title
            body
          }
        }
        `,
      }),
      transformResponse: (response: PostResponse) => response.data.post,
    }),
  }),
})

export const { useGetPostQuery } = gqlApi

// {"query":"query ($id: ID!) {post(id: 1) {title body}}"}
// {"operationName":null,"variables":{},"query":"{post(id: 1) {title body}}"}
