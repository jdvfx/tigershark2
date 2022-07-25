# Tigershark (v2)
is a CLI based 3D asset version tracking tool with simple CRUD functions using MongoDB

- create
- update
- get_latest (latest version)
- get_source (path of file that created the asset)
- delete (tag for deletion, a separate tool does the actual deletion)


each asset is stored in MongoDB as such:

Asset{ <br>
<t>name: asset_name <br>
  location: show/seq/shot <br>
  versions{ <br>
    0{ <br>
      version: 1 <br>
      datapath: my_file_path <br>
      source: file_that_created_the_asset <br>
      approved: false <br> 
      status: Online <br>
    } <br>
    1{ <br>
      version: 2 <br>
      ... <br>
    } <br>
  } <br>
} <br>

