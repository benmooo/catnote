import SearchIcon from '@mui/icons-material/Search'
import MeiliSearchIcon from '@mui/icons-material/SavedSearch'
export default (props: { show: boolean }) => {
  return (
    <div
      className={`${
        props.show ? '' : 'hidden'
      } z-30 fixed inset-0 bg-ink-t1 backdrop-blur-sm bg-opacity-50 flex justify-center items-center`}
    >
      <div className='w-full md:w-2/3 lg:w-1/2 bg-white p-6 rounded-md'>
        <div className='flex items-center'>
          <span>
            <SearchIcon sx={{ fontSize: 24 }}></SearchIcon>
          </span>
          <input
            type='text'
            placeholder='Search..'
            className='grow border-none focus:ring-transparent font-montserrat font-medium'
          />
          <span className='px-2 py-1 bg-ink-t1 text-ink-t4 rounded-lg'>
            Esc
          </span>
        </div>

        <div className='pt-4 px-2'>
          <div className='font-montserrat pb-2'>Recent</div>

          {[...Array(3).keys()].map((i) => (
            <div key={i} className='font-light text-sm py-1'>
              Searh Item {i}
            </div>
          ))}
        </div>

        <div className='pt-4 px-2'>
          <div className='font-montserrat pb-2'>Notes</div>

          {[...Array(3).keys()].map((i) => (
            <div
              key={i}
              className={`${
                i == 1 ? 'bg-ink text-ink-t4 rounded-md p-2' : ''
              } font-light text-sm py-1`}
            >
              Notes Item {i}
            </div>
          ))}
        </div>

        <div className='pt-4 px-2'>
          <div className='font-montserrat pb-2'>Tags</div>

          {[...Array(2).keys()].map((i) => (
            <div key={i} className='font-light text-sm py-1'>
              Tags Item {i}
            </div>
          ))}
        </div>

        <div className='flex justify-end items-center mt-12'>
          <span className='font-montserrat text-ink-t1 text-sm'>
            Powered by
          </span>
          <span className='pl-4'>
            <MeiliSearchIcon sx={{ fontSize: 24 }}></MeiliSearchIcon>
          </span>
          <span className='font-montserrat font-medium text-md'>
            Meilisearch
          </span>
        </div>
      </div>
    </div>
  )
}
