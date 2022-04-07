# Current state 

`cargo run -- user token url job_name`

will list the job as follows:

```
BuildInfo { id: 151, success: true, time: 329831 }
BuildInfo { id: 150, success: true, time: 165846 }
BuildInfo { id: 149, success: true, time: 226555 }
BuildInfo { id: 148, success: false, time: 279240 }
BuildInfo { id: 147, success: false, time: 264459 }
BuildInfo { id: 146, success: true, time: 245906 }
BuildInfo { id: 145, success: false, time: 373084 }
BuildInfo { id: 144, success: false, time: 318166 }
BuildInfo { id: 143, success: true, time: 379886 }
BuildInfo { id: 142, success: true, time: 240835 }
BuildInfo { id: 141, success: true, time: 288377 }
BuildInfo { id: 140, success: true, time: 255308 }
BuildInfo { id: 139, success: false, time: 150470 }
BuildInfo { id: 138, success: false, time: 152874 }
BuildInfo { id: 137, success: false, time: 139708 }
BuildInfo { id: 136, success: true, time: 247852 }
BuildInfo { id: 135, success: true, time: 175807 }
BuildInfo { id: 134, success: false, time: 178503 }
BuildInfo { id: 133, success: true, time: 252090 }
BuildInfo { id: 132, success: false, time: 134039 }
BuildInfo { id: 131, success: false, time: 114034 }
BuildInfo { id: 130, success: false, time: 128930 }
BuildInfo { id: 129, success: true, time: 250676 }
BuildInfo { id: 128, success: true, time: 326685 }
BuildInfo { id: 127, success: true, time: 191301 }
```

# Goal

Store that, monitor our job easily, and produce some meaningful stats.