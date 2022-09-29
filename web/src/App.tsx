import { Outlet, Route, Routes } from 'react-router-dom'

import Home from './routes/home'
import About from './routes/about'
import Signup from './routes/signup'
import Login from './routes/login'
import Notes from './routes/notes'
import Note from './routes/note'
import PageNotFound from './routes/404'

import Counter from './components/Counter'
import Todos from './components/Todos'

function App() {
  return (
    <Routes>
      <Route path='/' element={<Outlet></Outlet>}>
        <Route path='home' element={<Home />}></Route>
        <Route path='about' element={<About />}></Route>
        <Route path='signup' element={<Signup />}></Route>
        <Route path='login' element={<Login />}></Route>

        {/* testing */}
        <Route path='todos' element={<Todos />}></Route>
        <Route path='counter' element={<Counter />}></Route>

        <Route path='notes' element={<Outlet />}>
          <Route index element={<Notes />}></Route>
          <Route path=':noteId' element={<Note />}></Route>
        </Route>

        <Route path='*' element={<PageNotFound />}></Route>
      </Route>
    </Routes>
  )
}

export default App
