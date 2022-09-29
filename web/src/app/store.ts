import { configureStore } from '@reduxjs/toolkit'
import { api } from './services/auth'
import { jsonApi } from './services/todos'
import counterReducer from '../features/counter/counterSlice'
import authReducer from '../features/auth/authSlice'
import { gqlApi } from './services/posts'

export const store = configureStore({
  reducer: {
    counter: counterReducer,
    auth: authReducer,

    [api.reducerPath]: api.reducer,
    [jsonApi.reducerPath]: jsonApi.reducer,
    [gqlApi.reducerPath]: gqlApi.reducer,
  },
  middleware: (defaultMiddleware) =>
    defaultMiddleware().concat(api.middleware).concat(jsonApi.middleware).concat(gqlApi.middleware),
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch
