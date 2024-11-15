import cpp

// Predicate to find if a function is called only by main and is in source files
predicate calledOnlyByMain(Function f) {
  exists(FunctionCall call |
    call.getTarget() = f and
    call.getEnclosingFunction().getName() = "main"
  ) and
  not exists(FunctionCall call2 |
    call2.getTarget() = f and
    call2.getEnclosingFunction().getName() != "main"
  )
}

// Predicate to find if a function is unused and is in source files
predicate isUnused(Function f) {
  not f.getName() = "main" and
  not exists(FunctionCall call | 
    call.getTarget() = f
  )
}

// Select functions that are either called only by main or are unused, excluding library functions
from Function f, int i, Type t
where 
  (calledOnlyByMain(f) or isUnused(f)) and
  f.getFile().getBaseName().matches("%.c") and
  i < f.getNumberOfParameters() and
  t = f.getParameter(i).getType()
select f, f.getName(), i, t
