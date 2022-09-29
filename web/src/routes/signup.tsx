import CopyrightIcon from '@mui/icons-material/Copyright'
import logo from '../assets/images/logo.svg'

export default () => {
  return (
    <div className='h-screen bg-ink-t4 relative'>
      {/* login form */}
      <div className='pt-48'>
        <div className='flex justify-center items-center mb-8'>
          <img src={logo} alt='' />
          <div className='font-montserrat text-3xl pl-3'>SIGN UP</div>
        </div>

        <div className='flex justify-center p-3'>
          <input
            type='email'
            className='font-montserrat w-72 bg-ink-t4 border-ink-t1 border-t-0 border-x-0 focus:border-ink focus:ring-transparent px-0'
            placeholder='EMAIL:'
          />
        </div>

        <div className='flex justify-center p-3'>
          <input
            type='password'
            className='font-montserrat w-72 bg-ink-t4 border-ink-t1 border-t-0 border-x-0 focus:border-ink focus:ring-transparent px-0'
            placeholder='PASSWORD:'
          />
        </div>

        <div className='flex justify-center pt-8'>
          <button className='font-montserrat rounded-3xl shadow-md bg-ink px-6 text-ink-t4 py-1 text-lg hover:bg-ink-t4 hover:text-ink active:bg-ink-t3'>
            SIGN UP
          </button>
        </div>

        <div className='flex justify-center pt-4'>
          <p className='font-montserrat text-sm text-ink-t1 underline hover:text-ink'>
            <a>Already have an account, Login</a>
          </p>
        </div>
      </div>

      <div className='font-montserrat text-xs text-center absolute inset-x-0 bottom-0 pb-2'>
        <CopyrightIcon sx={{ fontSize: 12 }}></CopyrightIcon> CATNOTE 2020
      </div>
    </div>
  )
}
