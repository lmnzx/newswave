import { Routes, Route } from 'react-router-dom'
import Home from './Home'
import PageNotFound from './404'
import SignUp from './SignUp'
import Pub from './PublishNewsLetter'

export default function App() {
  return (
    <>
      <Routes>
        <Route path='/' element={<Home />} />
        <Route path='/signup' element={<SignUp />} />
        <Route path='/publish' element={<Pub />} />
        <Route path='*' element={<PageNotFound />} />
      </Routes>
    </>
  )
}

